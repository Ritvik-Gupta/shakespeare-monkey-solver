from time import time

from data_visualization import data_visualization as dv

start = time()
for elm in dv.compute_generations_for_dataset(
    "Hello World", (1, 5), (100, 150), (dv.BiasedScaleType.Order, 2.43)
):
    print(elm)
print(time() - start)
