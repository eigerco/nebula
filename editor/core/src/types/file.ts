export type ProjectFile = {
  name: string;
  id: number;
  content: string;
  children?: ProjectFile[];
};
