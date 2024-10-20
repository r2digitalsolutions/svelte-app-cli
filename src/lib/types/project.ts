export type TProject = {
  name: string;
  path: string;
  directory_name: string;
  version: string;
};

export type TProjects = TProject & {
  id: number | string;
}

export type TProjectDepency = {
  name: string;
  version: string;
}

export interface IProject extends TProjects {
  openDirectory(): void;
  getLocales(): Promise<string[]>;
  getDependencies(): Promise<TProjectDepency[]>;
  createLocale(locale: string): Promise<boolean>;
}