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
          sh 'cargo build'
        }
      }
    }
    stage('Test') {
      steps {
        checkout scm
        container('rust') {
          sh 'cargo install cargo-tarpaulin'
          withCredentials([string(credentialsId: 'alpaca_secret_key', variable: 'alpaca_secret_key')]) {
            withCredentials([string(credentialsId: 'alpaca_access_key', variable: 'alpaca_access_key')]) {
              withCredentials([string(credentialsId: 'coveralls_alpaca_client', variable: 'coveralls_alpaca_client')]) {
                sh 'cargo test'
                sh 'cargo tarpaulin --coveralls ${coveralls_alpaca_client}'
              }
            }
          }
        }
      }
    }
    stage('Release') {
      when {
        expression {
          params.RELEASE_SOLID == true
        }
      }
      steps {
        container('rust') {
          withCredentials([string(credentialsId: 'cargo_login_token', variable: 'cargo_login_token')]) {
            sh 'cargo login ' + cargo_login_token
            sh 'cargo publish'
          }
        }
      }
    }
  }
}