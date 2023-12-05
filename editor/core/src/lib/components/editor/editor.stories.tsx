import React from "react";
import { objectValuesToControls } from "../../storybook-utils";
import { Meta, StoryObj } from "@storybook/react";
import { StoryFn } from "@storybook/react";
import { Editor, EditorProps } from "../index";
import { ProjectManager } from "../../project-manager";

const meta: Meta<typeof Editor> = {
  title: "Default",
  component: Editor,
  argTypes: {},
};
export default meta;

const Template: StoryFn<typeof Editor> = (args: EditorProps) => (
  <Editor {...args} />
);

const PrimaryWithDefault = (props: EditorProps) => {
  return (
    <div style={{ height: "90vh" }}>
      <Editor {...props} />
    </div>
  );
};

export const Primary: StoryObj = {
  render: () => <PrimaryWithDefault {...defaultArgs} />,
};

const defaultArgs = {
    manager: new ProjectManager("Workspace1"),
    actions: [],
    editable: false,
    fileId: 1
  }

Primary.args = {
  ...defaultArgs
};
