import { usePopper } from 'react-popper'
import React, { useState } from 'react'
import { Button } from './Button'

export const Test = () => {
  const [referenceElement, setReferenceElement] = useState(null)
  const [popperElement, setPopperElement] = useState(null)
  const [arrowElement, setArrowElement] = useState(null)
  const { styles, attributes } = usePopper(referenceElement, popperElement, {
    modifiers: [
      {
        name: 'arrow',
        options: {
          element: arrowElement
        }
      }
    ]
  })

  return (
    <>
      <Button ref={setReferenceElement} label='Button' />

      <div ref={setPopperElement} style={styles.popper} {...attributes.popper}>
        Popper element
        <div ref={setArrowElement} style={styles.arrow}></div>
      </div>
    </>
  )
}
