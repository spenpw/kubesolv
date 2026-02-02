import kopf
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("kubesolv-operator")


@kopf.on.create("kubesolv.io", "v1", "solvers")
def create_solver(spec, **kwargs):
    logger.info(f"Creating solver with spec: {spec}")


@kopf.on.delete("kubesolv.io", "v1", "solvers")
def delete_solver(spec, **kwargs):
    logger.info(f"Deleting solver with spec: {spec}")


@kopf.on.create("kubesolv.io", "v1", "cubestates")
def create_cubes_tate(spec, **kwargs):
    logger.info(f"Creating cube state with spec: {spec}")


@kopf.on.delete("kubesolv.io", "v1", "cubestates")
def delete_cubes_tate(spec, **kwargs):
    logger.info(f"Deleting cube state with spec: {spec}")


@kopf.on.create("kubesolv.io", "v1", "solvejobs")
def create_solve_job(spec, **kwargs):
    logger.info(f"Creating solve job with spec: {spec}")


@kopf.on.delete("kubesolv.io", "v1", "solvejobs")
def delete_solve_job(spec, **kwargs):
    logger.info(f"Deleting solve job with spec: {spec}")
