from typing import Optional


class Dir:
    def __init__(self, root: Optional['Dir'], name: str, size: int, dirs: Optional[list['Dir']]) -> None:
        self.__root = root
        self.__name = name
        self.__size = size
        self.__dirs = dirs

    def size(self):
        self.__size = self.__calc_size(self.__dirs)
        return self.__size

    def dirs(self):
        return self.__dirs

    def add_dir(self, dir: 'Dir'):
        if self.__dirs is None:
            self.__dirs = [dir]
        else:
            self.__dirs.append(dir)

    def __calc_size(self, dirs: Optional[list['Dir']]) -> int:
        if dirs is None:
            return self.__size

        return self.__size + sum(map(lambda dir: dir.size(), dirs))


def main():
    dir = Dir(None, '/', 0, [])
    dir.add_dir(Dir(dir, 'a', 23, None))
    dir.add_dir(Dir(dir, 'b', 45, None))

    print(dir.size())


main()
