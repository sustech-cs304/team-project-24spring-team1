from flask import Flask, request, send_from_directory
import uuid
import subprocess
import os
# ignore missing import on code generation

app = Flask(__name__)

def generate_uuid():
    return str(uuid.uuid4())

# Image Uploading Target and Hosting Service

# Accept image upload from `/upload` endpoint
# and save to `./uploads/<uuid>.<ext>`.
# Next, invoke `cwebp` to convert it to webp to save storage.
# The webp image will be saved to `./uploads/<uuid>.webp`.
# Finally, return the webp image URL.
@app.route('/upload', methods=['POST'])
def upload():
    file = request.files['file']
    uuid = generate_uuid()
    ext = os.path.splitext(file.filename)[1]
    assert ext in ['.jpg', '.jpeg', '.png', '.gif']
    file.save(f'./uploads/{uuid}{ext}')

    subprocess.run(['cwebp', f'./uploads/{uuid}{ext}', '-o', f'./uploads/{uuid}.webp'])
    os.remove(f'./uploads/{uuid}{ext}')

    return f'/uploads/{uuid}.webp'

# 2. Serving the webp image from `/uploads/<uuid>.webp` endpoint.
# If the webp image does not exist, return 404.
@app.route('/uploads/<path:path>')
def serve_webp(path):
    return send_from_directory('./uploads', path,
                               mimetype='image/webp',
                               max_age=3600 * 24 * 7)
