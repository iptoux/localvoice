import type { Meta, StoryObj } from "@storybook/react";
import { Button } from "../components/ui/button";
import { Mic, Settings, Copy, Trash2, Download } from "lucide-react";

const meta: Meta<typeof Button> = {
  title: "UI/Button",
  component: Button,
  tags: ["autodocs"],
  parameters: {
    layout: "centered",
    backgrounds: {
      default: "dark",
    },
  },
};

export default meta;

export const Default: StoryObj<typeof Button> = {
  args: {
    children: "Button",
  },
};

export const Outline: StoryObj<typeof Button> = {
  args: {
    variant: "outline",
    children: "Outline Button",
  },
};

export const Secondary: StoryObj<typeof Button> = {
  args: {
    variant: "secondary",
    children: "Secondary Button",
  },
};

export const Ghost: StoryObj<typeof Button> = {
  args: {
    variant: "ghost",
    children: "Ghost Button",
  },
};

export const Destructive: StoryObj<typeof Button> = {
  args: {
    variant: "destructive",
    children: "Destructive Button",
  },
};

export const Link: StoryObj<typeof Button> = {
  args: {
    variant: "link",
    children: "Link Button",
  },
};

export const Small: StoryObj<typeof Button> = {
  args: {
    size: "sm",
    children: "Small Button",
  },
};

export const Large: StoryObj<typeof Button> = {
  args: {
    size: "lg",
    children: "Large Button",
  },
};

export const Icon: StoryObj<typeof Button> = {
  args: {
    size: "icon",
    children: <Mic size={16} />,
  },
};

export const IconSmall: StoryObj<typeof Button> = {
  args: {
    size: "icon-sm",
    children: <Settings size={14} />,
  },
};

export const WithIcon: StoryObj<typeof Button> = {
  args: {
    children: (
      <>
        <Copy size={14} data-slot="icon" />
        Copy
      </>
    ),
  },
};

export const Loading: StoryObj<typeof Button> = {
  args: {
    children: "Loading...",
    disabled: true,
  },
};

export const AllVariants: StoryObj = {
  parameters: {
    layout: "padded",
  },
  render: () => (
    <div className="flex flex-wrap gap-4 items-center">
      <Button variant="default">Default</Button>
      <Button variant="outline">Outline</Button>
      <Button variant="secondary">Secondary</Button>
      <Button variant="ghost">Ghost</Button>
      <Button variant="destructive">Destructive</Button>
      <Button variant="link">Link</Button>
    </div>
  ),
};

export const AllSizes: StoryObj = {
  parameters: {
    layout: "padded",
  },
  render: () => (
    <div className="flex flex-wrap gap-4 items-center">
      <Button size="xs">XS</Button>
      <Button size="sm">SM</Button>
      <Button size="default">Default</Button>
      <Button size="lg">LG</Button>
      <Button size="icon"><Mic size={16} /></Button>
      <Button size="icon-sm"><Settings size={14} /></Button>
      <Button size="icon-xs"><Trash2 size={12} /></Button>
      <Button size="icon-lg"><Download size={18} /></Button>
    </div>
  ),
};

export const Disabled: StoryObj = {
  parameters: {
    layout: "padded",
  },
  render: () => (
    <div className="flex flex-wrap gap-4 items-center">
      <Button disabled>Default Disabled</Button>
      <Button variant="outline" disabled>Outline Disabled</Button>
      <Button variant="secondary" disabled>Secondary Disabled</Button>
      <Button variant="destructive" disabled>Destructive Disabled</Button>
    </div>
  ),
};
