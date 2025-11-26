(set-logic HORN)

(declare-datatypes ((listOfInt 0)) (((conslistOfInt  (headlistOfInt Int) (taillistOfInt listOfInt)) (nillistOfInt ))))

(declare-fun |sorted| ( listOfInt ) Bool)

(assert
  (forall ( (v_1 listOfInt) ) 
    (=>
      (and
         (= v_1 nillistOfInt)
      )
      (sorted v_1)
    )
  )
)
(assert
  (forall ( (A listOfInt) (B Int) ) 
    (=>
      (and
       (= A (conslistOfInt B nillistOfInt))
      )
      (sorted A)
    )
  )
  )

(assert
  (forall ( (A listOfInt) (B Int) (C Int) (D listOfInt) (E listOfInt) ) 
    (=>
     (and
	  (= D (conslistOfInt C E))
	  (sorted D)
      (<= B C)
	  (= A (conslistOfInt B D))
      )
      (sorted A)
    )
  )
)

(assert
  (forall ( (A listOfInt) (B listOfInt) (C Int) (D Int) (E listOfInt) ) 
    (=>
     (and
	  (= A (conslistOfInt D E))
      (sorted A)
      (= B (conslistOfInt C A))
	  (> C D)
      )
      (not (sorted B))
    )
  )
)

(assert
  (forall ( (A listOfInt) (B listOfInt) (C Int) (D Int) (E listOfInt) ) 
    (=>
      (and
        (not (sorted A))
        (= B (conslistOfInt C A))
		(= A (conslistOfInt D E))
      )
      (not (sorted B))
    )
  )
)

(assert
  (forall ( (A listOfInt) ) 
    (=>
      (and
       (sorted A)
	   (not (sorted A))
      )
      false
    )
  )
)

(assert
  (forall ( (A listOfInt) (D Int) (E listOfInt) ) 
    (=>
     (and
	  (= A (conslistOfInt D E))
      (sorted A)
	  (not (sorted E)))
      false
    )
  )
)

(check-sat)
(exit)
