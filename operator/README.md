# Kubesolv Operator

The kubesolv operator is the component responsible for watching kubesolv CRDs, launching solver jobs, and recording the results.

## Custom Resource Definitions

### Solver

The Solver CRD is used to represent an "installed" Rubik's Cube solver algorithm. The Kubesolv project provides a framework for developing these algorithms and encapsulating them in an OCI image such that they are compatible with this resource. After developing an algorithm image, it can simply be installed by capturing its name, configuration information, image reference, and other information in a CR and applying it to the cluster. The cluster will then be capable of solving CubeStates using the new method.

### CubeState

The CubeState CRD is used to represent the state of a Rubik's Cube, usually to solve using installed Solvers.

### SolveJob

The SolveJob CRD is used to represent a request to solve a CubeState with an installed Solver. It contains a reference to the CubeState as well as a reference to the Solver to use. When a SolveJob is created, the operator will launch the actual solver depending on its configuration, and provide the referenced CubeState as an input. When the SolveJob finishes, the operator will record the results in the SolveJob status field, including stats such as time to solve, resources consumed, solutions generated, etc.
