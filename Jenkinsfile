podTemplate(yaml: """
apiVersion: v1
kind: Pod
metadata:
  labels:
    some-label: builder
spec:
  containers:
  - name: rust
    image: rust:1.47-buster
    command:
    - cat
    tty: true
"""
  )
pipeline {
  agent none
  stages {
    stage("test and build") {
      steps {
        withCredentials([string(credentialsId: 'alpaca_secret_key', variable: 'alpaca_secret_key')]) {
          withCredentials([string(credentialsId: 'alpaca_access_key', variable: 'alpaca_access_key')]) {
            checkout scm
            container('rust') {
              sh 'cargo test'
              sh 'cargo build --release'
            }
          }
        }
      }
    }
  }
}