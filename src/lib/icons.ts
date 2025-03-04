type Icons = {
  [key: string]: string;
};

const icons: Icons = {

}

export const getIcon = (name: string) => icons[name];
