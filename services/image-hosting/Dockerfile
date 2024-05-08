FROM python:3-slim

# Set the working directory
WORKDIR /app

# COPY cwebp
COPY cwebp /usr/bin/cwebp

# Install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt -i https://mirrors.sustech.edu.cn/pypi/web/simple

# Copy the source code
COPY server.py .

# Configure Flask and Gunicorn
ARG FLASK_DEBUG="false"
ENV FLASK_DEBUG="${FLASK_DEBUG}" \
    FLASK_APP="server.py"

# Expose the port
EXPOSE 8000

# Run the application
CMD ["gunicorn", "-b", "0.0.0.0", "server:app"]
