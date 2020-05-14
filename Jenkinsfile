node {
    checkout scm
    stage('Build') {
        docker.withRegistry('https://docker.pkg.github.com', 'remote-gateway-publish-credentials') {
            docker.build("docker.pkg.github.com/rustic-music-player/remote-gateway/remote-gateway:latest", './packages/remote-gateway').push()

            docker.build("docker.pkg.github.com/rustic-music-player/remote-gateway/proxy:latest", './packages/proxy').push()
        }
    }
}
