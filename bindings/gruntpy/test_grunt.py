from gruntpy import PyAxis;

axis = PyAxis.generate_axis_on_edges(1, 10)

print(axis.start)
print(axis.end)
print(axis.step)
print(axis.blocks_count)
print(axis.blocks_centers)
print(axis.blocks_edges)
