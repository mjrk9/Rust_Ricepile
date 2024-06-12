import matplotlib.pyplot as plt 
import pickle as pkl
import numpy as np
from log_bin import logbin

heights = []
avalanches = []
Ls = [8, 16, 32, 64, 128, 256, 512, 1024]

# for i in range(len(Ls)):
#     with open(f'heights_zeroth_{i}.bin', 'rb') as f:
#         zero_heights = pkl.load(f)
#     heights.append(zero_heights)

# plt.figure()
# plt.ylabel('Height of zeroth column')
# plt.xlabel('Grain added')
# for idx, height in enumerate(heights):  
#     plt.plot(height, label = f'L = {Ls[idx]}')

# plt.legend()
# plt.savefig(f'plots/full_heights.png')
# plt.show()


for i in range(len(Ls)):
    with open(f'avalanches_{i}.bin', 'rb') as f:
        ava = pkl.load(f)
    avalanches.append(ava)

plt.figure()
plt.ylabel('Avalanche')
plt.xlabel('Grain added')
for idx, s in enumerate(avalanches):  
    x, y = logbin(s, scale=1.2)
    plt.loglog(x, y, label = f'L = {Ls[idx]}')

plt.legend()
plt.savefig(f'plots/avalanches.png')
plt.show()