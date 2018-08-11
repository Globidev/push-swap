from random import shuffle, seed
from sys import argv

size = int(argv[1])
try:
    seed(int(argv[2]))
except:
    pass

data_set = list(range(size))
shuffle(data_set)

print(' '.join(f'{i}' for i in data_set))
