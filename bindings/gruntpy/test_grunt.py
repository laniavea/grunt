from gruntpy import PyAxis
from gruntpy import PyParams3D

axis = PyAxis.generate_axis_on_edges(1, 10)
axis_v2 = PyAxis.generate_axis_on_edges(-100000, 100000, 1)

params = PyParams3D(axis, axis_v2)

ax_y = params.axis_y

_ = ax_y.blocks_centers

print(params.axis_y.blocks_count)
