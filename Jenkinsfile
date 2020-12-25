podTemplate(containers: [
containerTemplate(name: 'rust', image: 'rust:1.47-buster', ttyEnabled: true, command: 'cat'),
]) {
  node(POD_LABEL) {
  parameters {
          booleanParam(name: 'RELEASE_SOLID', defaultValue: false, description: 'removes -SNAPSHOT, releases solid version to nexus and commits new SNAPSHOT version to GHE')
      }
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