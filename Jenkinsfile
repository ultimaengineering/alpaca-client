podTemplate(containers: [
containerTemplate(name: 'rust', image: 'rust:1.47-buster', ttyEnabled: true, command: 'cat'), ]) {
  node(POD_LABEL) {
    stage('build and test') {
      checkout scm
      container('rust') {
        stage('build') {
          sh 'cargo build --release'
        }
        stage('test') {
          withCredentials([string(credentialsId: 'alpaca_secret_key', variable: 'alpaca_secret_key')]) {
            withCredentials([string(credentialsId: 'alpaca_access_key', variable: 'alpaca_access_key')]) {
              sh 'cargo test'
            }
          }
        }
      }
    }
  }
}