def images = dockerImages()

node {
    checkout scm
    stage('Build') {
        docker.withRegistry('docker.pkg.github.com/rustic-music-player/remote-gateway', 'remote-gateway-publish-credentials') {
            parallel(images)
        }
    }
}

def dockerImages() {
    def images = ['remote-gateway': {
        stage('Remote Gateway') {
            def gatewayImage = docker.build("remote-gateway:latest")
            gatewayImage.push()
        }
    }]
    return images
}
