import AvatarBauhaus from './Avatars/Bauhaus'
import AvatarMarble from './Avatars/Marble'
import AvatarSunset from './Avatars/Sunset'
import AvatarPixel from './Avatars/Pixel'
import AvatarBeam from './Avatars/Beam'
import AvatarRing from './Avatars/Ring'
import PropTypes from 'prop-types'
import React from 'react'

interface AvatarProps {
  /**
   * An array of colors
   */
  colors?: string[]
  /**
   * The name used to generate the avatar
   */
  name: string
  /**
   * The size of the avatar
   */
  size?: number
  /**
   * Should this avatar be a square?
   */
  square?: boolean
  /**
   * The type of avatar used
   */
  variant?: 'pixel' | 'bauhaus' | 'ring' | 'beam' | 'sunset' | 'marble'
}

const variants = ['pixel', 'bauhaus', 'ring', 'beam', 'sunset', 'marble']

/**
 * The default avatar used when a user doesn't have one
 */
const Avatar = ({
  colors = ['#92a1c6', '#146a7c', '#f0ab3d', '#c271b4', '#c20d90'],
  name = 'Clara Barton',
  size = 40,
  square = false,
  variant = 'marble',
  ...props
}: AvatarProps) => {
  const avatarProps = { colors, name, size, square, ...props }
  const checkedVariant = () => {
    if (variants.includes(variant)) {
      return variant
    }
    return 'marble'
  }
  const avatars = {
    pixel: <AvatarPixel {...avatarProps} />,
    bauhaus: <AvatarBauhaus {...avatarProps} />,
    ring: <AvatarRing {...avatarProps} />,
    beam: <AvatarBeam {...avatarProps} />,
    sunset: <AvatarSunset {...avatarProps} />,
    marble: <AvatarMarble {...avatarProps} />
  }
  return avatars[checkedVariant()]
}

Avatar.propTypes = {
  variant: PropTypes.oneOf(variants)
}

export default Avatar
