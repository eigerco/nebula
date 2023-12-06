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

const WithDefault = (props: EditorProps) => {
  return (
    <div style={{ height: "90vh" }}>
      <Editor {...props} />
    </div>
  );
};

export const Primary: StoryObj = {
  render: () => <WithDefault {...defaultArgs} />,
};

const defaultArgs = {
  manager: new ProjectManager("Workspace1"),
  actions: [],
  editable: false,
  fileId: 1,
};

Primary.args = {
  ...defaultArgs,
};

export const SingleFile: StoryObj = {
  render: () => {
    let manager = new ProjectManager("Workspace2");
    manager.createEmbedFileStructure();
    return <WithDefault {...defaultArgs} manager={manager} editable={false} />;
  },
};
