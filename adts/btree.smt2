(set-option :produce-proofs true)
(set-logic HORN)

(declare-datatypes
 ((treeOfInt 0)) (
				  (
				   (nodetreeOfInt
					(value Int) (lefttreeOfInt treeOfInt) (righttreeOfInt treeOfInt))
				   (leaftreeOfInt))))

;; Binary search tree (BSTree)
(declare-fun |is-BSTree| (treeOfInt) Bool)

;; If a BTree is empty then it is a BSTree
(assert
 (forall
  ((T treeOfInt))
  (=>
   (= T leaftreeOfInt)
   (is-BSTree T))))

;; If a BTree has only one element then it is a BSTRee
(assert
 (forall
  ((T treeOfInt) (leftT treeOfInt) (rightT treeOfInt) (A Int))
  (=>
   (and
	(= leftT leaftreeOfInt)
	(= rightT leaftreeOfInt)
	(= T (nodetreeOfInt A leftT rightT)))
   (is-BSTree T))))

;; If a BTree has only a left branch, and the left branch is BSTree,
;; and the element are sorted; then it is a BSTree
(assert
 (forall
  ((T treeOfInt) (leftT treeOfInt) (lleftT treeOfInt) (lrightT treeOfInt) (rightT treeOfInt) (A Int) (B Int))
  (=>
   (and
	(= leftT (nodetreeOfInt B lleftT lrightT))
	(= rightT leaftreeOfInt)
	(= T (nodetreeOfInt A leftT rightT))
	(is-BSTree leftT)
	(> A B))
   (is-BSTree T))))

;; If a BTree has only a left branch, and, or the left branch is not BSTree,
;; or the element are not sorted; then it is not a BSTree
(assert
 (forall
  ((T treeOfInt) (leftT treeOfInt) (lleftT treeOfInt) (lrightT treeOfInt) (rightT treeOfInt) (A Int) (B Int))
  (=>
   (and
	(= leftT (nodetreeOfInt B lleftT lrightT))
	(= rightT leaftreeOfInt)
	(= T (nodetreeOfInt A leftT rightT))
	(not
	 (and
	  (is-BSTree leftT)
	  (> A B))))
   (not (is-BSTree T)))))

;; If a BTree has only a right branch, and the right branch is BSTree,
;; and the element are sorted; then  it is a BSTree
(assert
 (forall
  ( (T treeOfInt) (leftT treeOfInt) (rightT treeOfInt) (rleftT treeOfInt) (rrightT treeOfInt) (A Int) (C Int))
  (=>
   (and
	(= leftT leaftreeOfInt)
	(= rightT (nodetreeOfInt C rleftT rrightT))
	(= T (nodetreeOfInt A leftT rightT))
	(is-BSTree rightT)
	(< A C))
   (is-BSTree T))))

;; If a BTree has only a right branch, and, or the right branch is not BSTree,
;; or the element are not sorted; then it is not a BSTree
(assert
 (forall
  ( (T treeOfInt) (leftT treeOfInt) (rightT treeOfInt) (rleftT treeOfInt) (rrightT treeOfInt) (A Int) (C Int))
  (=>
   (and
	(= leftT leaftreeOfInt)
	(= rightT (nodetreeOfInt C rleftT rrightT))
	(= T (nodetreeOfInt A leftT rightT))
	(not
	 (and
	  (is-BSTree rightT)
	  (< A C))))
   (not (is-BSTree T)))))

;; If a BTree has two non non-leaf branches, and the branches are BSTree,
;; and the three values are sorted; then the Btree is a BSTRee
(assert
 (forall
  ( (T treeOfInt) (leftT treeOfInt) (lleftT treeOfInt) (lrightT treeOfInt) (rightT treeOfInt) (rleftT treeOfInt) (rrightT treeOfInt) (A Int) (B Int) (C Int))
  (=>
   (and
	(= leftT (nodetreeOfInt B lleftT lrightT))
	(= rightT (nodetreeOfInt C rleftT rrightT))
	(= T (nodetreeOfInt A leftT rightT))
	(is-BSTree leftT)
	(is-BSTree rightT)
	(> A B)	
	(< A C)
	)
   (is-BSTree T))))

;; If a BTree has two non non-leaf branches, and the branches are BSTree,
;; and the three values are sorted; then the Btree is a BSTRee
(assert
 (forall
  ( (T treeOfInt) (leftT treeOfInt) (lleftT treeOfInt) (lrightT treeOfInt) (rightT treeOfInt) (rleftT treeOfInt) (rrightT treeOfInt) (A Int) (B Int) (C Int))
  (=>
   (and
	(= leftT (nodetreeOfInt B lleftT lrightT))
	(= rightT (nodetreeOfInt C rleftT rrightT))
	(= T (nodetreeOfInt A leftT rightT))
	(not
	 (and
	  (is-BSTree leftT)
	  (is-BSTree rightT)))
	(not
	 (and
	  (> A B)	
	  (< A C)))
	)
   (not (is-BSTree T)))))

;; If at least one sub-tree is not a BSTree and the whole BTree is a BSTree;
;; then we have a bug
(assert
 (forall
  ( (T treeOfInt) (leftT treeOfInt) (rightT treeOfInt) (A Int))
  (=>
   (and
	(not
	 (and
	  (is-BSTree leftT)
	  (is-BSTree rightT)))
	(= T (nodetreeOfInt A leftT rightT))
	(is-BSTree T)
	)
   false)))

;; If at least one sub-tree is not a BSTree but the term are sorted and
;; the whole tree is a BSTRee; then we have a bug
(assert
 (forall
  ( (T treeOfInt) (leftT treeOfInt) (rightT treeOfInt) (A Int) (B Int) (C Int) (ll treeOfInt) (lr treeOfInt) (rl treeOfInt) (rr treeOfInt))
  (=>
   (and
	(= leftT (nodetreeOfInt B ll lr))
	(= rightT (nodetreeOfInt C rl rr))
	(not
	 (and
	  (is-BSTree leftT)
	  (is-BSTree rightT)))
	(= T (nodetreeOfInt A leftT rightT))
	(> A B)
	(< A C)
	(is-BSTree T))
   false)))

;; If at least one term is not sorted and the whole BTree is a BSTree;
;; then we have a bug
(assert
 (forall
  ( (T treeOfInt) (leftT treeOfInt) (rightT treeOfInt) (A Int) (B Int) (C Int) (ll treeOfInt) (lr treeOfInt) (rl treeOfInt) (rr treeOfInt))
  (=>
   (and
	(= leftT (nodetreeOfInt B ll lr))
	(= rightT (nodetreeOfInt C rl rr))
	(is-BSTree leftT)
	(is-BSTree rightT)
	(= T (nodetreeOfInt A leftT rightT))
	(not
	 (and
	  (> A B)
	  (< A C)))
	(is-BSTree T))
   false)))

;; If the left sub-tree is not a leaf and it is not a BSTree or the
;; element are not sorted and the final Tree is a BSTree then we have a bug
(assert
 (forall
  ( (T treeOfInt) (leftT treeOfInt) (rightT treeOfInt) (A Int) (B Int) (ll treeOfInt) (lr treeOfInt))
  (=>
   (and
	(= leftT (nodetreeOfInt B ll lr))
	(= rightT leaftreeOfInt)
	(= T (nodetreeOfInt A leftT rightT))
	(not
	 (and
	  (> A B)
	  (is-BSTree leftT)))
	(is-BSTree T))
   false)))

;; If the right sub-tree is not a leaf and it is not a BSTree or the
;; element are not sorted and the final Tree is a BSTree then we have a bug
(assert
 (forall
  ( (T treeOfInt) (leftT treeOfInt) (rightT treeOfInt) (A Int) (C Int) (rl treeOfInt) (rr treeOfInt))
  (=>
   (and
	(= rightT (nodetreeOfInt C rl rr))
	(= leftT leaftreeOfInt)
	(= T (nodetreeOfInt A leftT rightT))
	(not
	 (and
	  (< A C)
	  (is-BSTree rightT)))
	(is-BSTree T))
   false)))

(check-sat)
(exit)
