import styled from 'styled-components'
import React from 'react'

interface ButtonProps {
  /**
   * Is this the principal call to action on the page?
   */
  primary?: boolean
  /**
   * Button contents
   */
  label: string
  /**
   * Optional click handler
   */
  onClick?: () => void
}

const ButtonElement = styled.button`
  border: 0;
  border-radius: 3em;
  cursor: pointer;
  display: inline-block;
  font-family: 'Nunito Sans', 'Helvetica Neue', Helvetica, Arial, sans-serif;
  font-size: 14px;
  font-weight: 700;
  line-height: 1;
  padding: 11px 20px;
  transition: 0.2s;
`

const PrimaryButton = styled(ButtonElement)`
  background-color: #0ea5e9;
  color: #fff;

  &:hover {
    background-color: #0284c7;
  }
`

const RegularButton = styled(ButtonElement)`
  background-color: transparent;
  box-shadow: rgba(0, 0, 0, 0.15) 0px 0px 0px 1px inset;
  color: #000;
  mix-blend-mode: screen;

  &:hover {
    background-color: #000;
    color: #fff;
  }
`

/**
 * The default button used throughout the application
 */
export const Button = ({ label, primary = false, ...props }: ButtonProps) => {
  if (primary) {
    return (
      <PrimaryButton type='button' {...props}>
        {label}
      </PrimaryButton>
    )
  } else {
    return (
      <RegularButton type='button' {...props}>
        {label}
      </RegularButton>
    )
  }
}
