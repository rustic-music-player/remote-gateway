node {
    checkout scm
    stage('Build') {
        docker.withRegistry('docker.pkg.github.com/rustic-music-player/remote-gateway', 'remote-gateway-publish-credentials') {
            def gatewayImage = docker.build("remote-gateway:latest", './packages/remote-gateway')
            gatewayImage.push()

            def proxyImage = docker.build("proxy:latest", './packages/proxy')
            proxyImage.push()
        }
    }
}
