import { ComponentStory, ComponentMeta } from '@storybook/react'
import Avatar from '../components/Avatar'
import React from 'react'

export default {
  component: Avatar,
  title: 'Components/DefaultAvatar'
} as ComponentMeta<typeof Avatar>

const Template: ComponentStory<typeof Avatar> = (args) => <Avatar {...args} />

export const Bauhaus = Template.bind({})
Bauhaus.args = {
  name: 'User',
  variant: 'bauhaus'
}

export const Beam = Template.bind({})
Beam.args = {
  name: 'User',
  variant: 'beam'
}

export const Marble = Template.bind({})
Marble.args = {
  name: 'User',
  variant: 'marble'
}

export const Pixel = Template.bind({})
Pixel.args = {
  name: 'User',
  variant: 'pixel'
}

export const Ring = Template.bind({})
Ring.args = {
  name: 'User',
  variant: 'ring'
}

export const Sunset = Template.bind({})
Sunset.args = {
  name: 'User',
  variant: 'sunset'
}
