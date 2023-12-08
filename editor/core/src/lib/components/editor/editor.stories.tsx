import React from "react";
import { objectValuesToControls } from "../../storybook-utils";
import { Meta, StoryObj } from "@storybook/react";
import { StoryFn } from "@storybook/react";
import { Command, Editor, EditorProps } from "../index";
import { ProjectManager } from "../../project-manager";

const meta: Meta<typeof Editor> = {
  title: "Editor",
  component: Editor,
  argTypes: {},
};
export default meta;

const Template: StoryFn<typeof Editor> = (args: EditorProps) => (
  <Editor {...args} />
);

const WithDefault = (props: EditorProps) => {
  return (
    <div style={{ height: "100vh" }}>
      <Editor {...props} />
    </div>
  );
};

export const Primary: StoryObj = {
  render: () => <WithDefault {...defaultArgs} />,
};

const defaultArgs: EditorProps = {
  manager: new ProjectManager("Workspace1"),
  onEvent: (command: Command) => {
    console.log(command);
  },
  config: {
    editable: true,
    multiFile: true,
  },
  fileId: 3,
};

Primary.args = {
  ...defaultArgs,
};

export const SingleFile: StoryObj = {
  render: () => {
    let manager = new ProjectManager("Workspace2");
    manager.createEmbedFileStructure();
    return <WithDefault {...defaultArgs} manager={manager} fileId={1} config={{editable: false, multiFile: false}} />;
  },
};

export const MultiFile: StoryObj = {
  render: () => {
    let manager = new ProjectManager("Workspace3");
    manager.createDefaultFileStructure();
    return (
      <WithDefault
        {...defaultArgs}
        manager={manager}
        config={{ editable: true, multiFile: true }}
        fileId={3}
      />
    );
  },
};


export const LiveExample: StoryObj = {
  render: () => {
    let manager = new ProjectManager("Workspace3");
    manager.createEmbedFileStructure();
    const onEvent = (command: Command) => {
      //TODO: Call api
    }
    return (
      <WithDefault
        {...defaultArgs}
        manager={manager}
        onEvent={onEvent}
        config={{ editable: false, multiFile: false }}
        fileId={1}
      />
    );
  },
};
