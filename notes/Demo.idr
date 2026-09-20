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

data Ungrouped : UseTree -> Type where NameWitness :  Ungrouped (Name _)
                                       GlobWitness :  Ungrouped Glob
                                       PathWitness :  {auto 0 _ : Ungrouped t}
                                                   -> Ungrouped (Path _ t)

data UngroupedItems : FfiItems -> Type where
  ItemsEmptyWitness :  UngroupedItems (MkFfiItems _ _ _ [])
  ItemsNameWitness  :  UngroupedItems (MkFfiItems _ _ _ ((Name _) :: _))
  ItemsGlobWitness  :  UngroupedItems (MkFfiItems _ _ _ (Glob :: _))
  ItemsPathWitness  :  {auto 0 _ : Ungrouped t}
                    -> UngroupedItems (MkFfiItems _ _ _ ((Path _ t) :: _))

empty : String -> FfiItems
empty s = MkFfiItems s [] [] []

normalize : FfiItems -> (i : FfiItems ** UngroupedItems i)
normalize it = let nit = { uses $= foldr f [] } it in (nit ** _) where
  -- [NOTE]: this function requires asserting to the totality checker that the
  -- trees rooted at group imports are always bound to be smaller than the trees
  -- rooted one level above. This is because the shape of the `UseTree` type in
  -- group imports stops "growing" when it finds lists. The items of these lists
  -- (themselves trees) could then be found to be potentially larger than the
  -- top-level parent group. This is a contradiction because a path's tail's
  -- group import is always smaller than the sum of the path's tail and the
  -- path's head.
  f : UseTree -> List UseTree -> List UseTree
  f = (++) . f' where
    f' : UseTree -> List UseTree
    f' (Name id)     = [ Name id ]
    f' Glob          = [ Glob ]
    f' o@(Path id t) = map (\t => Path id t) (f' t)
    f' o@(Group l)   = map (\t => f' $ assert_smaller o t) l |> join

-- resolveOne : (i : FfiItems ** UngroupedItems i) -> List Resolution
-- resolveOne a@(it ** _) =
resolveOne : _ -> List Resolution
resolveOne _ =
  [] where
  -- join . (map $ \(u ** _) => resolveReexport u u it) . ex $ a where
    data T : Type where Mod  : FfiItems -> T
                        Item : String -> T

    ex : (i : FfiItems ** UngroupedItems i) -> List (u : UseTree ** Ungrouped u)
    ex ((MkFfiItems _ _ _ []) ** _)            =
      []
    ex a@(it@(MkFfiItems _ _ _ (h :: t)) ** _) =
      (h ** _) :: (ex $ assert_smaller a (({ uses := t } it) ** _))

    f : String -> FfiItems -> Maybe T
    f id (MkFfiItems _ is ms _) = find c ((map Item is) ++ (map Mod ms)) where
      c : T -> Bool
      c (Mod (MkFfiItems mid _ _ _)) = mid == id
      c (Item s) = s == id

    resolveReexport :  UseTree
                    -> (t : UseTree)
                    -> {auto 0 prf : Ungrouped t}
                    -> FfiItems
                    -> List Resolution
    resolveReexport oid (Name id) {prf = NameWitness} it@(MkFfiItems mid _ _ _)
      = case f id it of
             Just (Mod m)  =>
               [ Resolved oid ({ mods := [ m ] } . empty $ mid) ]
             Just (Item i) =>
               [ Resolved oid ({ items := [ i ] } . empty $ mid) ]
             Nothing       =>
               [ Unresolved oid ]
    resolveReexport oid Glob {prf = GlobWitness} it
      = [ Resolved oid it ]
    resolveReexport oid (Path id t) {prf = PathWitness} it
      = case f id it of Just (Mod m) => resolveReexport oid t m
                        Just _       => [ Unresolved oid ]
                        Nothing      => [ Unresolved oid ]

merge : (i : FfiItems ** UngroupedItems i) -> List Resolution -> FfiItems
merge (it ** _) []                                           = it
merge it@(_ ** _) ((Unresolved _) :: t)                      = merge it t
merge (it ** _) ((Resolved oid (MkFfiItems _ is ms _)) :: t) = merge nit t where
  nit : (i : FfiItems ** UngroupedItems i)
  nit = let base = { items $= (++ is)
                   , mods  $= (++ ms)
                   , uses  $= deleteBy f oid } it in (base ** _) where
                     f : UseTree -> UseTree -> Bool
                     f (Name id1) (Name id2) = id1 == id2
                     f Glob Glob = True
                     f (Path id1 t1) (Path id2 t2) = id1 == id2 && f t1 t2
                     f _ _ = False

covering
resolve : FfiItems -> FfiItems
resolve it = let nit = normalize . { mods $= map resolve } $ it in
                 (merge nit) . resolveOne $ nit
