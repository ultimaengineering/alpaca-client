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
  environment {
        alpaca_access_key = credentials('alpaca_access_key')
        alpaca_secret_key = credentials('alpaca_secret_key')
    }
  node(POD_LABEL) {
    stage('Build and test') {
    checkout scm
      container('rust') {
        sh 'cargo test'
        sh 'cargo build --release'
      }
    }
  }
}