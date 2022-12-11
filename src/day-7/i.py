from typing import Optional
from enum import Enum

DirType = Enum('DirType', ['DIR', 'FILE'])


class Dir:
    def __init__(self, root: Optional['Dir'], name: str, size: Optional[int], dirs: Optional[list['Dir']]) -> None:
        self.__root = root
        self.__name = name
        self.__size = size
        self.__dirs = dirs

    def root(self) -> Optional['Dir']:
        return self.__root

    def name(self) -> str:
        return self.__name

    def size(self) -> Optional[int]:
        if self.__size is None:
            self.__size = self.__calc_size(self.__dirs)

        return self.__size

    def dirs(self) -> Optional[list['Dir']]:
        return self.__dirs

    def add_dir(self, dir: 'Dir'):
        if self.__dirs is None:
            self.__dirs = [dir]
        else:
            self.__dirs.append(dir)

    def __calc_size(self, dirs: Optional[list['Dir']]) -> int:
        if self.__size is not None:
            return self.__size
        return sum(map(lambda dir: dir.size(), dirs))


def main():
    root = Dir(None, '/', None, [])
    current_dir = root

    with open('i.in', 'r') as f:
        for line in f.readlines():
            split = line.split(' ')
            first = split[0]
            match first:
                case '$':
                    command = split[1]
                    match command:
                        case 'ls':
                            pass
                        case 'cd':
                            target_dir = split[2].rstrip()
                            match target_dir:
                                case '/':
                                    pass
                                case '..':
                                    if current_dir is not None:
                                        current_dir = current_dir.root()
                                case _:
                                    if current_dir is not None:
                                        current_dir = next(
                                            filter(lambda dir: dir.name() == target_dir, current_dir.dirs()), None)

                case 'dir':
                    dir = split[1].rstrip()
                    current_dir.add_dir(Dir(current_dir, dir, None, []))
                case size_str:
                    size = int(size_str)
                    file = split[1].rstrip()
                    current_dir.add_dir(Dir(current_dir, file, size, None))

    print(root.size())


main()
