module Demo

import Data.List

%default total

data UseTree : Type where Path  : String -> UseTree -> UseTree
                          Name  : String -> UseTree
                          Glob  : UseTree
                          Group : List UseTree -> UseTree

-- [NOTE]: we do not currently consider item paths. We consider solely
-- identifiers. This applies to both `FfiItems` and its list of items
-- (themselves solely identifiers.)
record FfiItems where
  constructor MkFfiItems
  ident : String
  items : List String
  mods  : List FfiItems
  uses  : List UseTree

data Resolution : Type where Resolved   : UseTree -> FfiItems -> Resolution
                             Unresolved : UseTree -> Resolution

-- [NOTE]: the next two types are only used as proof witnesses and will not get
-- ported over to Rust. Instead, a new, refined type will be used in Rust to
-- express the constraints imposed by the below dependent types.

data Ungrouped : UseTree -> Type where NameWitness :  Ungrouped (Name _)
                                       GlobWitness :  Ungrouped Glob
                                       PathWitness :  {auto prf : Ungrouped t}
                                                   -> Ungrouped (Path _ t)

data UngroupedItems : FfiItems -> Type where
  EmptyWitness :  UngroupedItems (MkFfiItems _ _ _ [])
  Witness      :  {auto prfs : UngroupedItems (MkFfiItems mid is ms ts)}
               -> {auto prf : Ungrouped t}
               -> UngroupedItems (MkFfiItems mid is ms (t :: ts))

empty : String -> FfiItems
empty s = MkFfiItems s [] [] []

-- [NOTE]: this deconstructs a module that is proven to contain no group
-- imports, into its list of imports (also proven to not contain group imports.)
-- The inverse of this is done by the `re` function.
ex : (i : FfiItems ** UngroupedItems i) -> List (u : UseTree ** Ungrouped u)
ex ((MkFfiItems _ _ _ []) ** _)                              =
  []
ex a@(MkFfiItems mid is ms (h :: t) ** Witness {prfs} {prf}) =
  let na = ((MkFfiItems mid is ms t) ** prfs) in
      (h ** prf) :: (ex $ assert_smaller a na)

-- [NOTE]: this builds up a proof tree by deconstructing the already proven
-- trees of group-clean imports. The goal is to explain to the type-checker
-- that these imports will make up a module that is guaranteed to be clean of
-- group imports. The inverse of this is done by the `ex` function.
re :  List (u : UseTree ** Ungrouped u)
   -> FfiItems
   -> (i : FfiItems ** UngroupedItems i)
re [] (MkFfiItems mid is ms _) =
  (MkFfiItems mid is ms [] ** EmptyWitness)
re ((h ** hw) :: t) i          =
  let (MkFfiItems mid is ms us ** w) = re t i in
      (MkFfiItems mid is ms (h :: us) ** Witness {prfs = w} {prf = hw})

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
      [ ((Name id) ** NameWitness) ]
    f' Glob        =
      [ (Glob ** GlobWitness) ]
    f' (Path id t) =
      map (\(t ** w) => (Path id t ** PathWitness {prf = w})) (f' t)
    f' o@(Group l) =
      map (\t => f' $ assert_smaller o t) l |> join

resolveOne : (i : FfiItems ** UngroupedItems i) -> List Resolution
resolveOne a@(it ** _) =
  join . (map $ \(u ** prf) => resolveReexport u u {prf} it) . ex $ a where
    f : String -> FfiItems -> Maybe (Either String FfiItems)
    f id (MkFfiItems _ is ms _) = find c ((map Left is) ++ (map Right ms)) where
      c : Either String FfiItems -> Bool
      c (Left s) = s == id
      c (Right (MkFfiItems mid _ _ _)) = mid == id

    resolveReexport :  UseTree
                    -> (t : UseTree)
                    -> {auto 0 prf : Ungrouped t}
                    -> FfiItems
                    -> List Resolution
    resolveReexport oid (Name id) {prf = NameWitness} it@(MkFfiItems mid _ _ _)
      = case f id it of
             Just (Left i)  =>
               [ Resolved oid ({ items := [ i ] } . empty $ mid) ]
             Just (Right m) =>
               [ Resolved oid ({ mods := [ m ] } . empty $ mid) ]
             Nothing        =>
               [ Unresolved oid ]
    resolveReexport oid Glob {prf = GlobWitness} it
      = [ Resolved oid it ]
    resolveReexport oid (Path id t) {prf = PathWitness {prf}} it
      = case f id it of Just (Right m) => resolveReexport oid t {prf} m
                        Just _         => [ Unresolved oid ]
                        Nothing        => [ Unresolved oid ]

merge : (i : FfiItems ** UngroupedItems i) -> List Resolution -> FfiItems
merge (it ** _) []                                               = it
merge it@(_ ** _) ((Unresolved _) :: t)                          = merge it t
merge it@(iit ** _) ((Resolved oid (MkFfiItems _ is ms _)) :: t) = merge nit t
  where nit : (i : FfiItems ** UngroupedItems i)
        nit = let nus = (deleteBy f oid) . ex $ it in re nus iit where
          f : UseTree -> (u : UseTree ** Ungrouped u) -> Bool
          f (Name id1) (Name id2 ** _)                       =
            id1 == id2
          f Glob (Glob ** _)                                 =
            True
          f (Path id1 t1) (Path id2 t2 ** PathWitness {prf}) =
            id1 == id2 && f t1 (t2 ** prf)
          f _ _                                              =
            False

covering
resolve : FfiItems -> FfiItems
resolve it = let nit = normalize . { mods $= map resolve } $ it in
                 (merge nit) . resolveOne $ nit
