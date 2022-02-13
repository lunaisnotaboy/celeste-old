import React from 'react'
import '../../assets/styles/components/buttons.css'

interface ButtonProps {
  /**
   * Is this te principal call to action on the page?
   */
  primary?: boolean
  /**
   * How large should the button be?
   */
  size?: 'small' | 'medium' | 'large'
  /**
   * Button contents
   */
  label: string
  /**
   * Optional click handler
   */
  onClick?: () => void
}

/**
 * Primary UI component for user interaction
 */
export const Button = ({
  label,
  primary = false,
  size = 'medium',
  ...props
}: ButtonProps) => {
  const mode = primary ? 'button--primary' : 'button--regular'
  return (
    <button
      className={['button', `button--${size}`, mode].join(' ')}
      type='button'
      {...props}
    >
      {label}
    </button>
  )
}
