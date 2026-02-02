FROM python:3.13-slim-trixie
WORKDIR /app
COPY requirements.txt .
RUN --mount=type=cache,target=/root/.cache/pip pip install --no-cache-dir -r /app/requirements.txt
COPY . .
ENTRYPOINT [ "kopf", "run", "-A", "src/kubesolv/handlers.py" ]
