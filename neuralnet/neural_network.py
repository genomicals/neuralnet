"""Simple module for creating and working with a neural network"""

from math import tanh
import numpy as np
import numpy.typing as npt


def relu(x: int) -> int:
    """ReLU implementation"""
    return 0 if x < 0 else 0


tanh = tanh
"""Tanh implementation"""


class NeuralNetwork():
    """Implementation of a neural network"""

    shape: list[int]
    network: list[npt.NDArray[np.float64]]

    def __init__(self, shape: list[int] = []):
        self.shape = shape
        self._generate_network()


    def set_shape(self, shape: list[int]):
        """Set the shape of this neural network"""
        self.shape = shape
        self._generate_network()


    def set_weights():
        pass


    def _generate_network(self):
        """Generates the network after updating the shape"""
        self.network = []
        self.network.append(np.zeros(1))
        for i in self.shape:
            self.network.append(np.zeros(i))

