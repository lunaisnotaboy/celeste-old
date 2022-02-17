import { ComponentStory, ComponentMeta } from '@storybook/react'
import { Button } from '../components/Button'
import React from 'react'

export default {
  component: Button,
  title: 'Example/Button'
} as ComponentMeta<typeof Button>

const Template: ComponentStory<typeof Button> = (args) => <Button {...args} />

export const Primary = Template.bind({})
Primary.args = {
  label: 'Button',
  primary: true
}

export const Regular = Template.bind({})
Regular.args = {
  label: 'Button'
}
