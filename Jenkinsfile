pipeline {
  parameters {
    booleanParam(name: 'RELEASE_SOLID', defaultValue: false, description: 'release solid version.')
  }
  agent {
    kubernetes {
      yamlFile 'KubernetesBuilder.yaml'
    }
  }
  stages {
    stage('build and test') {
      steps {
        checkout scm
        container('rust') {
          sh 'cargo build --release'
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