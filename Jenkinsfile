podTemplate(yaml: """
apiVersion: v1
kind: Pod
metadata:
  labels:
    some-label: builder
spec:
  containers:
  - name: rust
    image: rust:1.43-stretch
    command:
    - cat
    tty: true
"""
  ) {
  node(POD_LABEL) {
    withCredentials([string(credentialsId: 'alpaca_secret_key', variable: 'alpaca_secret_key')]) {
      withCredentials([string(credentialsId: 'alpaca_access_key', variable: 'alpaca_access_key')]) {
        stage('Build and test') {
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