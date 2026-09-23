module Demo

import Data.List
import Data.List.Quantifiers
import Data.List1

%default total

namespace UseTree
  public export
  data UseTree : Type where Path  : String -> UseTree -> UseTree
                            Name  : String -> UseTree
                            Glob  : UseTree
                            Group : List UseTree -> UseTree

namespace RootUseTree
  public export
  data RootUseTree : Type where
    Path  :  (s : String)
          -> { auto prf : Not (s = "super") }
          -> RootUseTree
          -> RootUseTree
    Name  : (s : String) -> { auto prf : Not (s = "super") } -> RootUseTree
    Group : List RootUseTree -> RootUseTree
    Glob  : RootUseTree

ModulePath : Type
ModulePath = List String

data ModuleKind = Root | Node

determineModulePath : ModuleKind -> Type
determineModulePath Root = Void
determineModulePath Node = List1 String

determineModuleUseTree : ModuleKind -> Type
determineModuleUseTree Root = RootUseTree
determineModuleUseTree Node = UseTree

-- [NOTE]: we do not currently consider item paths. We consider solely
-- identifiers. This applies to both `FfiItems` and its list of items
-- (themselves solely identifiers.)
record FfiItems (s : ModuleKind) where
  constructor MkFfiItems
  path  : determineModulePath s
  ident : String
  items : List String
  mods  : List (FfiItems Node)
  uses  : List (determineModuleUseTree s)

data Resolution : Type where Resolved   : UseTree -> FfiItems -> Resolution
                             Unresolved : UseTree -> Resolution

record StatefulItems where
  constructor MkState
  state : FfiItems
  new   : FfiItems

-- [NOTE]: the next types are only used as proof witnesses and will not get
-- ported over to Rust. Instead, a new, refined type will be used in Rust to
-- express the constraints imposed by the below dependent types.

namespace Ungrouped
  public export
  data Ungrouped : UseTree -> Type where Name :  Ungrouped (Name _)
                                         Glob :  Ungrouped Glob
                                         Path :  {auto prf : Ungrouped t}
                                              -> Ungrouped (Path _ t)

  public export
  data UngroupedItems : FfiItems -> Type where
    Empty   :  UngroupedItems (MkFfiItems _ _ _ _ [])
    Witness :  {auto prfs : UngroupedItems (MkFfiItems mp mid is ms ts)}
            -> {auto prf : Ungrouped t}
            -> UngroupedItems (MkFfiItems mp mid is ms (t :: ts))

-- [NOTE]: this is just a forward definition of the proof that is given later on
-- to show that a module has a non-empty path. It is needed for the proof that a
-- given module is the root module, and using a mutual block often negatively
-- impacts the type-checker.
namespace NodeModule
  public export
  data NodeModule : FfiItems -> Type

namespace RootModule
  namespace Constraints
    export
    data IdUseTree : UseTree -> Type where Name : IdUseTree (Name _)
                                           Path : IdUseTree (Path _ _)

    export
    data NonIdUseTree : UseTree -> Type where Glob  : NonIdUseTree Glob
                                              Group : NonIdUseTree (Group _)

  public export
  data RootModule : FfiItems -> Type where
    Empty  :  RootModule (MkFfiItems [] "" _ [] [])
    Module :  { auto prfs : RootModule (MkFfiItems mp mid is ms us) }
           -> { auto prf : NodeModule nm }
           -> RootModule (MkFfiItems mp mid is (nm :: ms) us)
    Name   :  { auto prfs : RootModule (MkFfiItems mp mid is ms us) }
           -> { auto prf1 : Not (s = "super") }
           -> { auto prf2 : IdUseTree (Name s) }
           -> RootModule (MkFfiItems mp mid is ms ((Name s) :: us))
    Path   :  { auto prfs : RootModule (MkFfiItems mp mid is ms us) }
           -> { auto prf1 : Not (s = "super") }
           -> { auto prf2 : IdUseTree (Path s t) }
           -> RootModule (MkFfiItems mp mid is ms ((Path s t) :: us))
    Other  :  { auto prfs : RootModule (MkFfiItems mp mid is ms us) }
           -> { auto prf : NonIdUseTree o }
           -> RootModule (MkFfiItems mp mid is ms (o :: us))

namespace NodeModule
  public export
  data NodeModule : FfiItems -> Type where
    Empty  :  { auto prf : Not (s = "") }
           -> NodeModule (MkFfiItems (_ :: _) s _ [] _)
    Module :  { auto prfs : NodeModule (MkFfiItems mp mid is ms us) }
           -> { auto prf : NodeModule nm }
           -> NodeModule (MkFfiItems mp mid is (nm :: ms) us)

empty : ModulePath -> String -> FfiItems
empty p id = MkFfiItems p id [] [] []

-- [NOTE]: this deconstructs a module that is proven to contain no group
-- imports, into its list of imports (also proven to not contain group imports.)
-- The inverse of this is done by the `re` function.
ex : (i : FfiItems ** UngroupedItems i) -> List (u : UseTree ** Ungrouped u)
ex ((MkFfiItems _ _ _ _ []) ** _)                            =
  []
ex a@(MkFfiItems mp mid is ms (h :: t) ** Witness {prfs} {prf}) =
  let na = ((MkFfiItems mp mid is ms t) ** prfs) in
      (h ** prf) :: (ex $ assert_smaller a na)

-- [NOTE]: this builds up a proof tree by deconstructing the already proven
-- trees of group-clean imports. The goal is to explain to the type-checker
-- that these imports will make up a module that is guaranteed to be clean of
-- group imports. The inverse of this is done by the `ex` function.
re :  List (u : UseTree ** Ungrouped u)
   -> FfiItems
   -> (i : FfiItems ** UngroupedItems i)
re [] (MkFfiItems mp mid is ms _) =
  (MkFfiItems mp mid is ms [] ** Empty)
re ((h ** hw) :: t) i          =
  let (MkFfiItems mp mid is ms us ** w) = re t i in
      (MkFfiItems mp mid is ms (h :: us) ** Witness {prfs = w} {prf = hw})

normalize : FfiItems -> (i : FfiItems ** UngroupedItems i)
normalize it = let nl = (foldr f []) . uses $ it in re nl it where
  -- [NOTE]: this function requires asserting to the totality checker that the
  -- trees rooted at group imports are always bound to be smaller than the trees
  -- rooted one level above. This is because the shape of the `UseTree` type in
  -- group imports stops "growing" when it finds lists. The items of these lists
  -- (themselves trees) could then be found to be potentially larger than the
  -- top-level parent group. This is a contradiction because a path's tail's
  -- group import is always smaller than the sum of the path's tail and the
  -- path's head.
  f :  UseTree
    -> List (u : UseTree ** Ungrouped u)
    -> List (u : UseTree ** Ungrouped u)
  f = (++) . f' where
    f' : UseTree -> List (u : UseTree ** Ungrouped u)
    f' (Name id)   =
      [ ((Name id) ** Name) ]
    f' Glob        =
      [ (Glob ** Glob) ]
    f' (Path id t) =
      map (\(t ** w) => (Path id t ** Path {prf = w})) (f' t)
    f' o@(Group l) =
      map (\t => f' $ assert_smaller o t) l |> join

resolveOne : (i : FfiItems ** UngroupedItems i) -> FfiItems -> List Resolution
resolveOne a@(it ** _) st =
  let p := MkState { state = st, new = it }
  in join . (map $ \(u ** prf) => resolveReexport u u {prf} p) . ex $ a where
    f : String -> FfiItems -> Maybe (Either String FfiItems)
    f id (MkFfiItems _ _ is ms _) =
      find c ((map Left is) ++ (map Right ms)) where
        c : Either String FfiItems -> Bool
        c (Left s) = s == id
        c (Right (MkFfiItems _ mid _ _ _)) = mid == id

    trimSuper :  (t : UseTree ** Ungrouped t)
              -> (Nat, (t : UseTree ** Ungrouped t))
    trimSuper t = f t Z where
      f :  (t : UseTree ** Ungrouped t)
        -> Nat
        -> (Nat, (t : UseTree ** Ungrouped t))
      f (Path "super" nt ** Path {prf}) s = S s |> f (nt ** prf)
      f t@(_ ** _) s                             = (s, t)

    findSuper : FfiItems -> ModulePath -> FfiItems
    -- findSuper it p = let np := reverse . tail . reverse $ p
    --                  in ?rhs

    resolveReexport :  UseTree
                    -> (t : UseTree)
                    -> {auto 0 prf : Ungrouped t}
                    -> StatefulItems
                    -> List Resolution
    resolveReexport oid (Name id) {prf = Name}
                    (MkState { state = _, new = it@(MkFfiItems mp mid _ _ _) })
      = case f id it of
             Just (Left i)  =>
               [ oid |> Resolved $ { items := [ i ] } . (empty mp) $ mid ]
             Just (Right m) =>
               [ oid |> Resolved $ { mods := [ m ] } . (empty mp) $ mid ]
             Nothing        =>
               [ Unresolved oid ]
    resolveReexport oid Glob {prf = Glob}
                    (MkState { state = _, new = it })
      = [ Resolved oid it ]
    resolveReexport oid (Path id t) {prf = Path {prf}}
                    (MkState { state = st, new = it })
      = case f id it of Just (Right m) =>
                          let np := MkState { state = st, new = m }
                          in resolveReexport oid t {prf} np
                        Just _         => [ Unresolved oid ]
                        Nothing        => [ Unresolved oid ]

merge :  (i : FfiItems ** UngroupedItems i)
      -> List Resolution
      -> (i : FfiItems ** UngroupedItems i)
merge it@(_ ** _) []                                               = it
merge it@(_ ** _) ((Unresolved _) :: t)                            = merge it t
merge it@(iit ** _) ((Resolved oid (MkFfiItems _ _ is ms _)) :: t) = merge nit t
  where nit : (i : FfiItems ** UngroupedItems i)
        nit = let nus = (deleteBy f oid) . ex $ it
              in { items $= (++ is), mods $= (++ ms) } iit |> re nus where
                f : UseTree -> (u : UseTree ** Ungrouped u) -> Bool
                f (Name id1) (Name id2 ** _)                       =
                  id1 == id2
                f Glob (Glob ** _)                                 =
                  True
                f (Path id1 t1) (Path id2 t2 ** Path {prf}) =
                  id1 == id2 && f t1 (t2 ** prf)
                f _ _                                              =
                  False

covering
updateMod : FfiItems -> FfiItems -> FfiItems
updateMod nit@(MkFfiItems nmp _ _ _ _) st@(MkFfiItems mp _ _ _ _) =
  case nmp == mp of True  => nit
                    False => f nit st
  where
    f : FfiItems -> FfiItems -> FfiItems
    f nit@(MkFfiItems nmp _ _ _ _) st =
      { mods $= (foldr f' []) . (map (updateMod nit)) } $ st where
        f' : FfiItems -> List FfiItems -> List FfiItems
        f' it@(MkFfiItems mp _ _ _ _) l = case mp == nmp of True  => nit :: l
                                                            False => it :: l

covering
resolveDriver : StatefulItems -> StatefulItems
resolveDriver (MkState { state = st, new = it }) =
  let (nst, nms) := (foldr r (st, [])) . mods $ it
      nit        := normalize . { mods := nms } $ it
  in f nit nst where
    r :  FfiItems
      -> (FfiItems, List FfiItems)
      -> (FfiItems, List FfiItems)
    r it (st, its) =
      let (MkState { state = nst, new = nit }) :=
          resolveDriver (MkState { state = st , new = it })
      in (nst, nit :: its)

    f : (i : FfiItems ** UngroupedItems i) -> FfiItems -> StatefulItems
    f a@(it ** _) st = let na@(nit ** _) := st |> resolveOne a |> merge a
                           nst           := updateMod nit st
                       in case (length . uses $ it) == (length . uses $ nit) of
                               True  => MkState { state = nst, new = nit }
                               False => f na nst

covering
resolve : FfiItems -> FfiItems
resolve it = MkState it it |> resolveDriver |> new

-- [NOTE]: the following tests comprise only test data. To test it out, the
-- Idris REPL is required.

-- [NOTE]: this test is for the resolution function `resolve`.
-- test1 : FfiItems
-- test1 = let bar := { items := [ "Foo" ] } . (empty "foo::bar") $ "bar"
--             foo := { mods := [ bar ] } . (empty "foo") $ "foo"
--         in { mods := [ foo ]
--            , uses := [ Path "bar" Glob, Path "foo" Glob ] } . (empty "") $ ""

-- [NOTE]: this test is for the normalization function `normalize`.
-- test2 : FfiItems
-- test2 =
--   let g := [ Name "TypeId", Name "Any" ]
--   in { uses := [ Path "std" (Path "any" (Group g)) ] } . (empty "") $ ""

-- [NOTE]: this test is for the normalization function `normalize`.
-- test3 : FfiItems
-- test3 =
--   let g := [ Path "foo" (Path "bar" (Group [ Name "Ty"
--                                           , Path "test" (Name "foobar") ]))
--            , Name "TypeId"
--            , Name "Any" ]
--   in { uses := [ Path "std" (Path "any" (Group g)) ] } . (empty "") $ ""

-- [NOTE]: this test is for the resolution function `resolve`.
-- test4 : FfiItems
-- test4 =
--   let uses   := [ Path "std" (Path "os" (Path "raw" (Name "c_void")))
--                 , Path "level1" Glob ]
--       id     := "level1"
--       level1 := { items := [ "Foo", "bar", "Word" ] } . (empty id) $ id
--       mods   := [ level1 ]
--       items  := [ "Array", "baz", "malloc" ]
--   in { items := items, mods := mods, uses := uses } . (empty "") $ ""

-- [NOTE]: this test is for the resolution function `resolve`.
-- test5 : FfiItems
-- test5 =
--   let foobar := { items := [ "Foo" ] } . (empty "foo::bar::foobar") $ "foobar"
--       bar    := { mods := [ foobar ] } . (empty "foo::bar") $ "bar"
--       foo    := { mods := [ bar ] } . (empty "foo") $ "foo"
--   in { uses := [ Path "foobar" Glob, Path "bar" Glob, Path "foo" (Name "bar") ]
--      , mods := [ foo ] } . (empty "") $ ""
