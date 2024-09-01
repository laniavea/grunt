import time

from gruntpy import PyAxis
from gruntpy import PyParams3D

axis = PyAxis.generate_axis_on_edges(1, 10)
axis_v2 = PyAxis.generate_axis_on_edges(-100000, 100000, 0.002)

print("Axes created")
time.sleep(5)

params = PyParams3D(axis, axis_v2)

print("Params created")
time.sleep(5)

ax_y = params.axis_y
print("Ax created")
time.sleep(5)

_ = ax_y.blocks_centers

print("_ created")
time.sleep(5)

print(params.axis_y.blocks_count)
