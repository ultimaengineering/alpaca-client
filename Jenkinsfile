podTemplate(containers: [
    containerTemplate(name: 'rust', image: 'rust:1.47-buster', ttyEnabled: true, command: 'cat'),
  ]) {
    node(POD_LABEL) {
        stage('build and test') {
            container('rust') {
                stage('build') {
                    sh 'cargo build --release'
                }
                stage('test') { sh 'cargo test' }
            }
        }
    }
}