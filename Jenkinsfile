pipeline{
    agent{
        label any
    }
    tools{
        cargo 'cargo 1.91.0'
        rust 'rustc 1.91.0'

    }
    environment {
        AWS_REGION = 'us-east-1'
        AWS_ACCOUNT_ID = '810244486469'
        ECR_REPO = 'hollowkey'
        IMAGE_URI = "${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ECR_REPO}"
        IMAGE_TAG = "${env.BUILD_NUMBER}"
    }
    stages{
        
        stage(" Fetching Code from git :)"){
            steps{
                git branch: 'main', url: 'https://github.com/itsadijmbt/HollowKey'
                
        }post{
            success{
                echo "++++++++++ FETCHING SUCCESSFULL +++++++++++++++"
            }
            failure{
                echo "!!!!!!!!!!!!!! FETCHING FAILED !!!!!!!!!!!!!!!!"
            }
        }

      }

      stage ("Building the code :"){
          steps{

            sh '''
             source $HOME/.cargo/env
             cargo build --release

            '''
          }
      }post{
        success{
            echo "++++++++++ BUILD SUCCESSFULL +++++++++++++++"
        }
        failure{
            echo "!!!!!!!!!!!!!! BUILD FAILED !!!!!!!!!!!!!!!!"
        }
      }

      stage("Testing the build code"){
          steps{
              sh '''
             source $HOME/.cargo/env
             cargo test --

            '''
          }
          post{
              always{
                  echo "====++++always++++===="
              }
              success{
                  echo "====++++Testing the build code executed successfully++++===="
              }
              failure{
                  echo "====++++Testing the build code execution failed++++===="
              }
      
          }
      }
    
}