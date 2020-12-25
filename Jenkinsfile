pipeline {
  parameters {
    booleanParam(name: 'RELEASE_SOLID', defaultValue: false, description: 'release solid version.')
  }
  agent {
    kubernetes {
      yamlFile 'KubernetesPod.yaml'
    }
  }
  stages {
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