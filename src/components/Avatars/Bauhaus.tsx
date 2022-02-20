import {
  hashCode,
  getUnit,
  getRandomColor,
  getBoolean
} from '../../utils/avatar'
import React from 'react'

const ELEMENTS = 4
const SIZE = 80

function generateColors(name: string, colors: string[]) {
  const numFromName = hashCode(name)
  const range = colors && colors.length

  const elementsProperties = Array.from({ length: ELEMENTS }, (_, i) => ({
    color: getRandomColor(numFromName + i, colors, range),
    translateX: getUnit(numFromName * (i + 1), SIZE / 2 - (i + 17), 1),
    translateY: getUnit(numFromName * (i + 1), SIZE / 2 - (i + 17), 2),
    rotate: getUnit(numFromName * (i + 1), 360),
    isSquare: getBoolean(numFromName, 2)
  }))

  return elementsProperties
}

const AvatarBauhaus = (props: any) => {
  const properties = generateColors(props.name, props.colors)

  return (
    <svg
      xmlns='http://www.w3.org/2000/svg'
      fill='none'
      height={props.size}
      role='img'
      viewBox={'0 0 ' + SIZE + ' ' + SIZE}
      width={props.size}
    >
      <title>{props.name}</title>
      <mask
        id='mask__bauhaus'
        height={SIZE}
        maskUnits='userSpaceOnUse'
        width={SIZE}
        x={0}
        y={0}
      >
        <rect
          fill='#fff'
          height={SIZE}
          rx={props.square ? undefined : SIZE * 2}
          width={SIZE}
        />
      </mask>
      <g mask='url(#mask__bauhaus)'>
        <rect fill={properties[0].color} height={SIZE} width={SIZE} />
        <rect
          fill={properties[1].color}
          height={properties[1].isSquare ? SIZE : SIZE / 8}
          transform={
            'translate(' +
            properties[1].translateX +
            ' ' +
            properties[1].translateY +
            ') rotate(' +
            properties[1].rotate +
            ' ' +
            SIZE / 2 +
            ' ' +
            SIZE / 2 +
            ')'
          }
        />
        <circle
          cx={SIZE / 2}
          cy={SIZE / 2}
          fill={properties[2].color}
          r={SIZE / 5}
          transform={
            'translate(' +
            properties[2].translateX +
            ' ' +
            properties[2].translateY +
            ')'
          }
        />
        <line
          stroke={properties[3].color}
          strokeWidth={2}
          transform={
            'translate(' +
            properties[3].translateX +
            ' ' +
            properties[3].translateY +
            ') rotate(' +
            properties[3].rotate +
            ' ' +
            SIZE / 2 +
            ' ' +
            SIZE / 2 +
            ')'
          }
        />
      </g>
    </svg>
  )
}

export default AvatarBauhaus
