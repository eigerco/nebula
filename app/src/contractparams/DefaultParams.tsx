import React from 'react'

interface Props {
  updateParams: any
}

export class DefaultParams extends React.Component<Props> {
  constructor(props: any) {
    super(props)
    this.props.updateParams([])
  }
  render() {
    return <div></div>
  }
}
