module Demo

import Data.List

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

total
empty : String -> FfiItems
empty s = MkFfiItems s [] [] []

total
normalize : FfiItems -> FfiItems
normalize it = { uses $= foldr f [] } it where
  total
  f : UseTree -> List UseTree -> List UseTree
  f = (++) . f' where total f' : UseTree -> List UseTree
                      f' (Name id)   = [ Name id ]
                      f' Glob        = [ Glob ]
                      f' (Path id t) = assert_total map (\t => Path id t) (f' t)
                      f' (Group l)   = assert_total map f' l |> join

partial
resolveOne : FfiItems -> List Resolution
resolveOne it = join . (map $ \u => resolveReexport u u it) . uses $ it where
  data Ty = Mod FfiItems | Item String

  total
  f : String -> FfiItems -> Maybe Ty
  f id (MkFfiItems _ is ms _) = find c (map Item is ++ map Mod ms) where
    c : Ty -> Bool
    c (Mod (MkFfiItems mid _ _ _)) = mid == id
    c (Item s) = s == id

  -- [NOTE]: this function is not even covering because that would require
  -- making the `UseTree` type a GADT. That is not worth it for a PoC.
  partial
  resolveReexport : UseTree -> UseTree -> FfiItems -> List Resolution
  resolveReexport oid (Name id) it@(MkFfiItems mid _ _ _) =
    case f id it of
         Just (Mod m)  => [ Resolved oid (({ mods := [ m ] } . empty) mid) ]
         Just (Item i) => [ Resolved oid (({ items := [ i ] } . empty) mid) ]
         Nothing       => [ Unresolved oid ]
  resolveReexport oid Glob it                             = [ Resolved oid it ]
  resolveReexport oid (Path id t) it                      =
    case f id it of Just (Mod m) => resolveReexport oid t m
                    Just _       => [ Unresolved oid ]
                    Nothing      => [ Unresolved oid ]

total
merge : FfiItems -> List Resolution -> FfiItems
merge it [] = it
merge it ((Unresolved _) :: t) = merge it t
merge it ((Resolved oid (MkFfiItems _ is ms _)) :: t) = merge mit t where
  mit : FfiItems
  mit = { items $= (++ is)
        , mods  $= (++ ms)
        , uses  $= deleteBy f oid } it where
          -- [NOTE]: this function is semantically partial because it handles
          -- not the case for a group import. The wildcard case, which is used
          -- for combinations of the considered (partial) cases, makes the
          -- function covering even though it is meant to be partial. We use
          -- `deleteBy` instead of implementing `Eq` for `UseTree` because
          -- partial functions are painful enough.
          total
          f : UseTree -> UseTree -> Bool
          f (Name id1) (Name id2) = id1 == id2
          f Glob Glob = True
          f (Path id1 t1) (Path id2 t2) = id1 == id2 && f t1 t2
          f _ _ = False

partial
resolve : FfiItems -> FfiItems
resolve it = (merge nit) . resolveOne . { mods $= map resolve } $ nit where
  total
  nit : FfiItems
  nit = normalize it
