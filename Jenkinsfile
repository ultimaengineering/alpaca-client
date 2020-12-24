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
  ) {
  parameters {
          booleanParam(name: 'RELEASE_SOLID', defaultValue: false, description: 'removes -SNAPSHOT, releases solid version to nexus and commits new SNAPSHOT version to GHE') }
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
