module Demo

import Data.List
import Data.Vect

%default total

namespace UseTree
  public export
  data UseTree : Type where Path  : String -> UseTree -> UseTree
                            Name  : String -> UseTree
                            Glob  : UseTree
                            Group : List UseTree -> UseTree

ModulePath : Nat -> Type
ModulePath n = Vect n String

namespace FfiItems
  public export
  record FfiItems (n : Nat) where
    constructor MkFfiItems
    path  : Vect n String
    ident : String
    items : List String
    mods  : List (FfiItems (S n))
    uses  : List UseTree

record ResolvedItem where
  constructor MkResolved
  items : List String
  mods  : List (FfiItems depth)
  uses  : List UseTree

data Resolution : Type where
  Resolved   : UseTree -> ResolvedItem -> Resolution
  Unresolved : UseTree -> Resolution

record StatefulItems (n : Nat) where
  constructor MkState
  state : FfiItems Z
  current   : FfiItems n

-- [NOTE]: these types are only used as proof witnesses and will not get ported
-- over to Rust. Instead, a new, refined type will be used in Rust to express
-- the constraints imposed by the below dependent types.
namespace Ungrouped
  public export
  data Ungrouped : UseTree -> Type where Name :  Ungrouped (Name _)
                                         Glob :  Ungrouped Glob
                                         Path :  {auto prf : Ungrouped t}
                                              -> Ungrouped (Path _ t)

  public export
  data UngroupedItems : FfiItems _ -> Type where
    Empty   :  UngroupedItems (MkFfiItems _ _ _ _ [])
    Witness :  {auto prfs : UngroupedItems (MkFfiItems mp mid is ms ts)}
            -> {auto prf : Ungrouped t}
            -> UngroupedItems (MkFfiItems mp mid is ms (t :: ts))

empty : ModulePath n -> String -> FfiItems n
empty p id = MkFfiItems p id [] [] []

-- [NOTE]: this deconstructs a module that is proven to contain no group
-- imports, into its list of imports (also proven to not contain group imports.)
-- The inverse of this is done by the `re` function.
ex : (i : FfiItems _ ** UngroupedItems i) -> List (u : UseTree ** Ungrouped u)
ex ((MkFfiItems _ _ _ _ []) ** _)                            =
  []
ex a@(MkFfiItems mp mid is ms (h :: t) ** Witness {prfs} {prf}) =
  let na = ((MkFfiItems mp mid is ms t) ** prfs) in
      (h ** prf) :: (ex $ assert_smaller a na)

-- [NOTE]: this builds up a proof tree by deconstructing the already proven
-- trees of group-clean imports. The goal is to explain to the type-checker that
-- these imports will make up a module that is guaranteed to be clean of
-- group imports. The inverse of this is done by the `ex` function.
re :  List (u : UseTree ** Ungrouped u)
   -> FfiItems n
   -> (i : FfiItems n ** UngroupedItems i)
re [] (MkFfiItems mp mid is ms _) =
  (MkFfiItems mp mid is ms [] ** Empty)
re ((h ** hw) :: t) i          =
  let (MkFfiItems mp mid is ms us ** w) = re t i in
      (MkFfiItems mp mid is ms (h :: us) ** Witness {prfs = w} {prf = hw})

normalize : FfiItems n -> (i : FfiItems n ** UngroupedItems i)
normalize it@(MkFfiItems { uses = us, _ }) = foldr f [] us |> re <| it where
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
    f' (Name id)   = [ (Name id ** Name) ]
    f' Glob        = [ (Glob ** Glob) ]
    f' (Path id t) =         f' t |> flip map <| \(t ** _) => (  Path id t
                                                              ** Path)
    f' o@(Group l) = join <| l    |> flip map <| \t => f' $ assert_smaller o t

-- [TODO]: reimplement this.
findItem : String -> FfiItems n -> Maybe (Either String (FfiItems (S n)))

samePath : ModulePath _ -> ModulePath _ -> Bool
samePath []         []         = True
samePath (h1 :: t1) (h2 :: t2) = h1 == h2 && samePath t1 t2
samePath _          _          = False

covering
findParent : StatefulItems (S n) -> Maybe (FfiItems n)
findParent (MkState { state = st, current = (MkFfiItems cmp _ _ _ _) }) =
  f st where r : FfiItems _ -> Maybe (FfiItems n) -> Maybe (FfiItems n)
             f : FfiItems _ -> Maybe (FfiItems n)
             r it Nothing                  = f it
             r _  p                        = p
             f it@(MkFfiItems mp _ _ ms _) =
               case cmp |> samePath . reverse . tail . reverse <| mp of
                    True  => believe_me $ Just it
                    False => foldr r Nothing ms

namespace ResolveNode
  public export partial
  resolveReexport :  UseTree
                  -> (t : UseTree)
                  -> {auto 0 prf : Ungrouped t}
                  -> StatefulItems (S _)
                  -> List Resolution
  resolveReexport oid (Name id) st@(MkState { current = it, _ })
    = case id == "super" of
           True  => case findParent st of
                         Just m  =>
                           [ oid |> Resolved <| MkResolved { items = [ ]
                                                           , mods  = [ m ]
                                                           , uses  = [ ] } ]
                         Nothing =>
                           idris_crash "[TODO]"
           False => case findItem id it of
                         Just (Left i)  =>
                           [ oid |> Resolved <| MkResolved { items = [ i ]
                                                           , mods  = [ ]
                                                           , uses  = [ ]
                                                           , depth = Z } ]
                         Just (Right m) =>
                           [ oid |> Resolved <| MkResolved { items = [ ]
                                                           , mods  = [ m ]
                                                           , uses  = [ ] } ]
                         Nothing        =>
                           [ Unresolved oid ]
  resolveReexport oid Glob (MkState { current = (MkFfiItems { items = is
                                                            , mods  = ms
                                                            , uses  = us
                                                            , _ })
                                    , _ })
    = [ Resolved oid $ MkResolved { items = is, mods = ms, uses = us } ]
  resolveReexport oid (Path id t) {prf = Path {prf}}
                  ist@(MkState { state = st, current = it })
    = case id == "super" of
           True  => case findParent ist of
                         Just m  => ?rhspath
                         Nothing => idris_crash "[TODO]"
           False => case findItem id it of
                         Just (Right m) => ResolveNode.resolveReexport oid t $
                                             MkState { state   = st
                                                     , current = m }
                         Just _         => [ Unresolved oid ]
                         Nothing        => [ Unresolved oid ]

namespace ResolveRoot
  public export
  resolveReexport :  UseTree
                  -> (t : UseTree)
                  -> {auto 0 prf : Ungrouped t}
                  -> StatefulItems _
                  -> List Resolution
  resolveReexport oid (Name id) (MkState { current = it, _ })
    = case findItem id it of
           Just (Left i)  =>
             [ oid |> Resolved <| MkResolved { items = [ i ]
                                             , mods  = [ ]
                                             , uses  = [ ]
                                             , depth = Z } ]
           Just (Right m) =>
             [ oid |> Resolved <| MkResolved { items = [ ]
                                             , mods  = [ m ]
                                             , uses  = [ ] } ]
           Nothing        =>
             [ Unresolved oid ]
  resolveReexport oid Glob (MkState { current = (MkFfiItems { items = is
                                                            , mods  = ms
                                                            , uses  = us
                                                            , _ })
                                    , _ })
    = [ Resolved oid $ MkResolved { items = is, mods = ms, uses = us } ]
  resolveReexport oid (Path id t) {prf = Path {prf}} (MkState { state   = st
                                                              , current = it })
    = case findItem id it of
           Just (Right m) => ResolveRoot.resolveReexport oid t $
                               MkState { state = st, current = m }
           Just _         => [ Unresolved oid ]
           Nothing        => [ Unresolved oid ]

partial
resolveOne :  { n : _ }
           -> (i : FfiItems n ** UngroupedItems i)
           -> FfiItems Z
           -> List Resolution
resolveOne a@(it ** _) st = join . (map $ f) . ex $ a where
  p : StatefulItems n
  f : (u : UseTree ** Ungrouped u) -> List Resolution
  p          = MkState st it
  f (u ** _) = case n of Z   => ResolveRoot.resolveReexport u u p
                         S _ => ResolveNode.resolveReexport u u p



covering
merge :  (i : FfiItems n ** UngroupedItems i)
      -> List Resolution
      -> (i : FfiItems n ** UngroupedItems i)
merge it@(_ ** _)   []                                         = it
merge it@(_ ** _)   ((Unresolved _) :: t)                      = merge it t
merge it@(iit ** _) ((Resolved oid (MkResolved is ms _)) :: t) = merge nit t
  where
    covering
    transform :  FfiItems _
              -> (FfiItems m, List (FfiItems (S m)))
              -> (FfiItems m, List (FfiItems (S m)))
    transform it (st, its) = let np := st |> snoc . path <| ident it in
      ( st
      , { path := np
        , mods := mods it |>
                  foldr transform (empty np $ ident it, []) |> snd } it :: its)

    covering
    nit : (i : FfiItems n ** UngroupedItems i)
    nit =
      let nus = (deleteBy f oid) . ex $ it
      in { items $= (++ is)
         , mods  := foldr transform (iit, []) ms |> snd } iit |> re nus
      where
        f : UseTree -> (u : UseTree ** Ungrouped u) -> Bool
        f (Name id1)    (Name id2 ** _)             =
          id1 == id2
        f Glob          (Glob ** _)                 =
          True
        f (Path id1 t1) (Path id2 t2 ** Path {prf}) =
          id1 == id2 && f t1 (t2 ** prf)
        f _             _                           =
          False

-- [TODO]: implement this function correctly. Potentially use `StatefulItems`.
covering
updateMod : FfiItems n -> FfiItems Z -> FfiItems Z

partial
resolveDriver : { n : _ } -> StatefulItems n -> StatefulItems n
resolveDriver (MkState { state = st, current = it }) =
  let (nst, nms) := (foldr r (st, [])) . mods $ it
      nit        := normalize . { mods := nms } $ it
  in f nit nst where
    r :  FfiItems (S n)
      -> (FfiItems Z, List (FfiItems (S n)))
      -> (FfiItems Z, List (FfiItems (S n)))
    r it (st, its) =
      let (MkState { state = nst, current = nit }) :=
          resolveDriver (MkState { state = st , current = it })
      in (nst, nit :: its)

    f : (i : FfiItems n ** UngroupedItems i) -> FfiItems Z -> StatefulItems n
    f a@(it ** _) st =
      let na@(nit ** _) := st |> resolveOne a |> merge a
          nst           := updateMod nit st
      in case (List.length . uses $ it) == (List.length . uses $ nit) of
              True  => MkState { state = nst, current = nit }
              False => f na nst

partial
resolve : FfiItems Z -> FfiItems Z
resolve it = MkState it it |> resolveDriver |> current

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
