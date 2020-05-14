node {
    checkout scm
    docker.withRegistry('https://docker.pkg.github.com', 'github') {
        stage('Build Gateway') {
            docker.build("docker.pkg.github.com/rustic-music-player/remote-gateway/remote-gateway:latest", './packages/remote-gateway').push()
        }

        stage('Build Proxy') {
            docker.build("docker.pkg.github.com/rustic-music-player/remote-gateway/proxy:latest", './packages/proxy').push()
        }
    }
}
