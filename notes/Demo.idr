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
normalize it = let normalized = { uses $= foldr f [] } it in
                   (normalized ** _) where
  f : UseTree -> List UseTree -> List UseTree
  f = (++) . f' where
    -- [NOTE]: this function requires asserting to the totality checker that the
    -- trees rooted at paths and group imports are always bound to be smaller
    -- than the trees rooted one level above. This is not encoded in the
    -- `UseTree` type to keep things simple to port to Rust.
    f' : UseTree -> List UseTree
    f' (Name id)     = [ Name id ]
    f' Glob          = [ Glob ]
    f' o@(Path id t) = map (\t => Path id t) (f' $ assert_smaller o t)
    f' o@(Group l)   = map (\t => f' $ assert_smaller o t) l |> join

resolveOne : (i : FfiItems ** UngroupedItems i) -> List Resolution
resolveOne it = join . (map $ \u => resolveReexport u u it) . uses $ it where
  data Ty = Mod FfiItems | Item String

  f : String -> FfiItems -> Maybe Ty
  f id (MkFfiItems _ is ms _) = find c (map Item is ++ map Mod ms) where
    c : Ty -> Bool
    c (Mod (MkFfiItems mid _ _ _)) = mid == id
    c (Item s) = s == id

  resolveReexport :  UseTree
                  -> (t : UseTree)
                  -> {auto 0 prf : Ungrouped t}
                  -> FfiItems
                  -> List Resolution
  resolveReexport oid (Name id) {prf = NameWitness} it@(MkFfiItems mid _ _ _) =
    case f id it of
         Just (Mod m)  => [ Resolved oid ({ mods := [ m ] } . empty $ mid) ]
         Just (Item i) => [ Resolved oid ({ items := [ i ] } . empty $ mid) ]
         Nothing       => [ Unresolved oid ]
  resolveReexport oid Glob {prf = GlobWitness} it = [ Resolved oid it ]
  resolveReexport oid (Path id t) {prf = PathWitness} it =
    case f id it of Just (Mod m) => resolveReexport oid t m
                    Just _       => [ Unresolved oid ]
                    Nothing      => [ Unresolved oid ]

merge : (i : FfiItems ** UngroupedItems i) -> List Resolution -> FfiItems
merge (it ** _) [] = it
merge it ((Unresolved _) :: t) = merge it t
merge (it ** _) ((Resolved oid (MkFfiItems _ is ms _)) :: t) = merge mit t where
  mit : (i : FfiItems ** UngroupedItems i)
  mit = let base = { items $= (++ is)
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
