node {
    checkout scm
    stage('Build') {
        docker.withRegistry('https://docker.pkg.github.com', 'remote-gateway-publish-credentials') {
            def gatewayImage = docker.build("/rustic-music-player/remote-gateway/remote-gateway:latest", './packages/remote-gateway')
            gatewayImage.push()

            def proxyImage = docker.build("/rustic-music-player/remote-gateway/proxy:latest", './packages/proxy')
            proxyImage.push()
        }
    }
}
