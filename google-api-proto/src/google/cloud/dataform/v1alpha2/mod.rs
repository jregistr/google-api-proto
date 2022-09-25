/// Represents a Dataform Git repository.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Repository {
    /// Output only. The repository's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If set, configures this repository to be linked to a Git remote.
    #[prost(message, optional, tag="2")]
    pub git_remote_settings: ::core::option::Option<repository::GitRemoteSettings>,
}
/// Nested message and enum types in `Repository`.
pub mod repository {
    /// Controls Git remote configuration for a repository.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GitRemoteSettings {
        /// Required. The Git remote's URL.
        #[prost(string, tag="1")]
        pub url: ::prost::alloc::string::String,
        /// Required. The Git remote's default branch name.
        #[prost(string, tag="2")]
        pub default_branch: ::prost::alloc::string::String,
        /// Required. The name of the Secret Manager secret version to use as an
        /// authentication token for Git operations. Must be in the format
        /// `projects/*/secrets/*/versions/*`.
        #[prost(string, tag="3")]
        pub authentication_token_secret_version: ::prost::alloc::string::String,
        /// Output only. Indicates the status of the Git access token.
        #[prost(enumeration="git_remote_settings::TokenStatus", tag="4")]
        pub token_status: i32,
    }
    /// Nested message and enum types in `GitRemoteSettings`.
    pub mod git_remote_settings {
        /// Indicates the status of a Git authentication token.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum TokenStatus {
            /// Default value. This value is unused.
            Unspecified = 0,
            /// The token could not be found in Secret Manager (or the Dataform
            /// Service Account did not have permission to access it).
            NotFound = 1,
            /// The token could not be used to authenticate against the Git remote.
            Invalid = 2,
            /// The token was used successfully to authenticate against the Git remote.
            Valid = 3,
        }
        impl TokenStatus {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    TokenStatus::Unspecified => "TOKEN_STATUS_UNSPECIFIED",
                    TokenStatus::NotFound => "NOT_FOUND",
                    TokenStatus::Invalid => "INVALID",
                    TokenStatus::Valid => "VALID",
                }
            }
        }
    }
}
/// `ListRepositories` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepositoriesRequest {
    /// Required. The location in which to list repositories. Must be in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of repositories to return. The server may return fewer
    /// items than requested. If unspecified, the server will pick an appropriate
    /// default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListRepositories` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListRepositories`
    /// must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. This field only supports ordering by `name`. If unspecified, the server
    /// will choose the ordering. If specified, the default order is ascending for
    /// the `name` field.
    #[prost(string, tag="4")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Filter for the returned list.
    #[prost(string, tag="5")]
    pub filter: ::prost::alloc::string::String,
}
/// `ListRepositories` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepositoriesResponse {
    /// List of repositories.
    #[prost(message, repeated, tag="1")]
    pub repositories: ::prost::alloc::vec::Vec<Repository>,
    /// A token which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations which could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `GetRepository` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRepositoryRequest {
    /// Required. The repository's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// `CreateRepository` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRepositoryRequest {
    /// Required. The location in which to create the repository. Must be in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The repository to create.
    #[prost(message, optional, tag="2")]
    pub repository: ::core::option::Option<Repository>,
    /// Required. The ID to use for the repository, which will become the final component of
    /// the repository's resource name.
    #[prost(string, tag="3")]
    pub repository_id: ::prost::alloc::string::String,
}
/// `UpdateRepository` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRepositoryRequest {
    /// Optional. Specifies the fields to be updated in the repository. If left unset,
    /// all fields will be updated.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The repository to update.
    #[prost(message, optional, tag="2")]
    pub repository: ::core::option::Option<Repository>,
}
/// `DeleteRepository` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRepositoryRequest {
    /// Required. The repository's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true, any child resources of this repository will also be
    /// deleted. (Otherwise, the request will only succeed if the repository has no
    /// child resources.)
    #[prost(bool, tag="2")]
    pub force: bool,
}
/// `FetchRemoteBranches` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchRemoteBranchesRequest {
    /// Required. The repository's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// `FetchRemoteBranches` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchRemoteBranchesResponse {
    /// The remote repository's branch names.
    #[prost(string, repeated, tag="1")]
    pub branches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a Dataform Git workspace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workspace {
    /// Output only. The workspace's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// `ListWorkspaces` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkspacesRequest {
    /// Required. The repository in which to list workspaces. Must be in the
    /// format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of workspaces to return. The server may return fewer
    /// items than requested. If unspecified, the server will pick an appropriate
    /// default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListWorkspaces` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListWorkspaces`
    /// must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. This field only supports ordering by `name`. If unspecified, the server
    /// will choose the ordering. If specified, the default order is ascending for
    /// the `name` field.
    #[prost(string, tag="4")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Filter for the returned list.
    #[prost(string, tag="5")]
    pub filter: ::prost::alloc::string::String,
}
/// `ListWorkspaces` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkspacesResponse {
    /// List of workspaces.
    #[prost(message, repeated, tag="1")]
    pub workspaces: ::prost::alloc::vec::Vec<Workspace>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations which could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `GetWorkspace` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkspaceRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// `CreateWorkspace` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkspaceRequest {
    /// Required. The repository in which to create the workspace. Must be in the format
    /// `projects/*/locations/*/repositories/*`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The workspace to create.
    #[prost(message, optional, tag="2")]
    pub workspace: ::core::option::Option<Workspace>,
    /// Required. The ID to use for the workspace, which will become the final component of
    /// the workspace's resource name.
    #[prost(string, tag="3")]
    pub workspace_id: ::prost::alloc::string::String,
}
/// `DeleteWorkspace` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkspaceRequest {
    /// Required. The workspace resource's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the author of a Git commit.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitAuthor {
    /// Required. The commit author's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The commit author's email address.
    #[prost(string, tag="2")]
    pub email_address: ::prost::alloc::string::String,
}
/// `PullGitCommits` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullGitCommitsRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The name of the branch in the Git remote from which to pull commits.
    /// If left unset, the repository's default branch name will be used.
    #[prost(string, tag="2")]
    pub remote_branch: ::prost::alloc::string::String,
    /// Required. The author of any merge commit which may be created as a result of merging
    /// fetched Git commits into this workspace.
    #[prost(message, optional, tag="3")]
    pub author: ::core::option::Option<CommitAuthor>,
}
/// `PushGitCommits` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushGitCommitsRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The name of the branch in the Git remote to which commits should be pushed.
    /// If left unset, the repository's default branch name will be used.
    #[prost(string, tag="2")]
    pub remote_branch: ::prost::alloc::string::String,
}
/// `FetchFileGitStatuses` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchFileGitStatusesRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// `FetchFileGitStatuses` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchFileGitStatusesResponse {
    /// A list of all files which have uncommitted Git changes. There will only be
    /// a single entry for any given file.
    #[prost(message, repeated, tag="1")]
    pub uncommitted_file_changes: ::prost::alloc::vec::Vec<fetch_file_git_statuses_response::UncommittedFileChange>,
}
/// Nested message and enum types in `FetchFileGitStatusesResponse`.
pub mod fetch_file_git_statuses_response {
    /// Represents the Git state of a file with uncommitted changes.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UncommittedFileChange {
        /// The file's full path including filename, relative to the workspace root.
        #[prost(string, tag="1")]
        pub path: ::prost::alloc::string::String,
        /// Indicates the status of the file.
        #[prost(enumeration="uncommitted_file_change::State", tag="2")]
        pub state: i32,
    }
    /// Nested message and enum types in `UncommittedFileChange`.
    pub mod uncommitted_file_change {
        /// Indicates the status of an uncommitted file change.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum State {
            /// Default value. This value is unused.
            Unspecified = 0,
            /// The file has been newly added.
            Added = 1,
            /// The file has been deleted.
            Deleted = 2,
            /// The file has been modified.
            Modified = 3,
            /// The file contains merge conflicts.
            HasConflicts = 4,
        }
        impl State {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    State::Unspecified => "STATE_UNSPECIFIED",
                    State::Added => "ADDED",
                    State::Deleted => "DELETED",
                    State::Modified => "MODIFIED",
                    State::HasConflicts => "HAS_CONFLICTS",
                }
            }
        }
    }
}
/// `FetchGitAheadBehind` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchGitAheadBehindRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The name of the branch in the Git remote against which this workspace
    /// should be compared. If left unset, the repository's default branch name
    /// will be used.
    #[prost(string, tag="2")]
    pub remote_branch: ::prost::alloc::string::String,
}
/// `FetchGitAheadBehind` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchGitAheadBehindResponse {
    /// The number of commits in the remote branch that are not in the workspace.
    #[prost(int32, tag="1")]
    pub commits_ahead: i32,
    /// The number of commits in the workspace that are not in the remote branch.
    #[prost(int32, tag="2")]
    pub commits_behind: i32,
}
/// `CommitWorkspaceChanges` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitWorkspaceChangesRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The commit's author.
    #[prost(message, optional, tag="4")]
    pub author: ::core::option::Option<CommitAuthor>,
    /// Optional. The commit's message.
    #[prost(string, tag="2")]
    pub commit_message: ::prost::alloc::string::String,
    /// Optional. Full file paths to commit including filename, rooted at workspace root. If
    /// left empty, all files will be committed.
    #[prost(string, repeated, tag="3")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `ResetWorkspaceChanges` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetWorkspaceChangesRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Full file paths to reset back to their committed state including filename,
    /// rooted at workspace root. If left empty, all files will be reset.
    #[prost(string, repeated, tag="2")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. If set to true, untracked files will be deleted.
    #[prost(bool, tag="3")]
    pub clean: bool,
}
/// `FetchFileDiff` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchFileDiffRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The file's full path including filename, relative to the workspace root.
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
}
/// `FetchFileDiff` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchFileDiffResponse {
    /// The raw formatted Git diff for the file.
    #[prost(string, tag="1")]
    pub formatted_diff: ::prost::alloc::string::String,
}
/// `QueryDirectoryContents` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDirectoryContentsRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub workspace: ::prost::alloc::string::String,
    /// Optional. The directory's full path including directory name, relative to the
    /// workspace root. If left unset, the workspace root is used.
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    /// Optional. Maximum number of paths to return. The server may return fewer
    /// items than requested. If unspecified, the server will pick an appropriate
    /// default.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `QueryDirectoryContents` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `QueryDirectoryContents` must match the call that provided the page
    /// token.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// `QueryDirectoryContents` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDirectoryContentsResponse {
    /// List of entries in the directory.
    #[prost(message, repeated, tag="1")]
    pub directory_entries: ::prost::alloc::vec::Vec<query_directory_contents_response::DirectoryEntry>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Nested message and enum types in `QueryDirectoryContentsResponse`.
pub mod query_directory_contents_response {
    /// Represents a single entry in a workspace directory.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DirectoryEntry {
        #[prost(oneof="directory_entry::Entry", tags="1, 2")]
        pub entry: ::core::option::Option<directory_entry::Entry>,
    }
    /// Nested message and enum types in `DirectoryEntry`.
    pub mod directory_entry {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Entry {
            /// A file in the directory.
            #[prost(string, tag="1")]
            File(::prost::alloc::string::String),
            /// A child directory in the directory.
            #[prost(string, tag="2")]
            Directory(::prost::alloc::string::String),
        }
    }
}
/// `MakeDirectory` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MakeDirectoryRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The directory's full path including directory name, relative to the
    /// workspace root.
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
}
/// `MakeDirectory` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MakeDirectoryResponse {
}
/// `RemoveDirectory` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDirectoryRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The directory's full path including directory name, relative to the
    /// workspace root.
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
}
/// `MoveDirectory` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveDirectoryRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The directory's full path including directory name, relative to the
    /// workspace root.
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    /// Required. The new path for the directory including directory name, rooted at
    /// workspace root.
    #[prost(string, tag="3")]
    pub new_path: ::prost::alloc::string::String,
}
/// `MoveDirectory` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveDirectoryResponse {
}
/// `ReadFile` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadFileRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The file's full path including filename, relative to the workspace root.
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
}
/// `ReadFile` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadFileResponse {
    /// The file's contents.
    #[prost(bytes="bytes", tag="1")]
    pub file_contents: ::prost::bytes::Bytes,
}
/// `RemoveFile` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFileRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The file's full path including filename, relative to the workspace root.
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
}
/// `MoveFile` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveFileRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The file's full path including filename, relative to the workspace root.
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    /// Required. The file's new path including filename, relative to the workspace root.
    #[prost(string, tag="3")]
    pub new_path: ::prost::alloc::string::String,
}
/// `MoveFile` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveFileResponse {
}
/// `WriteFile` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteFileRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub workspace: ::prost::alloc::string::String,
    /// Required. The file.
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    /// Required. The file's contents.
    #[prost(bytes="bytes", tag="3")]
    pub contents: ::prost::bytes::Bytes,
}
/// `WriteFile` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteFileResponse {
}
/// `InstallNpmPackages` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallNpmPackagesRequest {
    /// Required. The workspace's name.
    #[prost(string, tag="1")]
    pub workspace: ::prost::alloc::string::String,
}
/// `InstallNpmPackages` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallNpmPackagesResponse {
}
/// Represents the result of compiling a Dataform project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompilationResult {
    /// Output only. The compilation result's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. If set, fields of `code_compilation_overrides` override the default
    /// compilation settings that are specified in dataform.json.
    #[prost(message, optional, tag="4")]
    pub code_compilation_config: ::core::option::Option<compilation_result::CodeCompilationConfig>,
    /// Output only. The version of `@dataform/core` that was used for compilation.
    #[prost(string, tag="5")]
    pub dataform_core_version: ::prost::alloc::string::String,
    /// Output only. Errors encountered during project compilation.
    #[prost(message, repeated, tag="6")]
    pub compilation_errors: ::prost::alloc::vec::Vec<compilation_result::CompilationError>,
    #[prost(oneof="compilation_result::Source", tags="2, 3")]
    pub source: ::core::option::Option<compilation_result::Source>,
}
/// Nested message and enum types in `CompilationResult`.
pub mod compilation_result {
    /// Configures various aspects of Dataform code compilation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CodeCompilationConfig {
        /// Optional. The default database (Google Cloud project ID).
        #[prost(string, tag="1")]
        pub default_database: ::prost::alloc::string::String,
        /// Optional. The default schema (BigQuery dataset ID).
        #[prost(string, tag="2")]
        pub default_schema: ::prost::alloc::string::String,
        /// Optional. The default BigQuery location to use. Defaults to "US".
        /// See the BigQuery docs for a full list of locations:
        /// <https://cloud.google.com/bigquery/docs/locations.>
        #[prost(string, tag="8")]
        pub default_location: ::prost::alloc::string::String,
        /// Optional. The default schema (BigQuery dataset ID) for assertions.
        #[prost(string, tag="3")]
        pub assertion_schema: ::prost::alloc::string::String,
        /// Optional. User-defined variables that are made available to project code during
        /// compilation.
        #[prost(btree_map="string, string", tag="4")]
        pub vars: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
        /// Optional. The suffix that should be appended to all database (Google Cloud project
        /// ID) names.
        #[prost(string, tag="5")]
        pub database_suffix: ::prost::alloc::string::String,
        /// Optional. The suffix that should be appended to all schema (BigQuery dataset ID)
        /// names.
        #[prost(string, tag="6")]
        pub schema_suffix: ::prost::alloc::string::String,
        /// Optional. The prefix that should be prepended to all table names.
        #[prost(string, tag="7")]
        pub table_prefix: ::prost::alloc::string::String,
    }
    /// An error encountered when attempting to compile a Dataform project.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompilationError {
        /// Output only. The error's top level message.
        #[prost(string, tag="1")]
        pub message: ::prost::alloc::string::String,
        /// Output only. The error's full stack trace.
        #[prost(string, tag="2")]
        pub stack: ::prost::alloc::string::String,
        /// Output only. The path of the file where this error occurred, if available, relative to
        /// the project root.
        #[prost(string, tag="3")]
        pub path: ::prost::alloc::string::String,
        /// Output only. The identifier of the action where this error occurred, if available.
        #[prost(message, optional, tag="4")]
        pub action_target: ::core::option::Option<super::Target>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Immutable. Git commit/tag/branch name at which the repository should be compiled.
        /// Must exist in the remote repository.
        /// Examples:
        /// - a commit SHA: `12ade345`
        /// - a tag: `tag1`
        /// - a branch name: `branch1`
        #[prost(string, tag="2")]
        GitCommitish(::prost::alloc::string::String),
        /// Immutable. The name of the workspace to compile. Must be in the format
        /// `projects/*/locations/*/repositories/*/workspaces/*`.
        #[prost(string, tag="3")]
        Workspace(::prost::alloc::string::String),
    }
}
/// `ListCompilationResults` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCompilationResultsRequest {
    /// Required. The repository in which to list compilation results. Must be in the
    /// format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of compilation results to return. The server may return
    /// fewer items than requested. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListCompilationResults` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCompilationResults`
    /// must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// `ListCompilationResults` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCompilationResultsResponse {
    /// List of compilation results.
    #[prost(message, repeated, tag="1")]
    pub compilation_results: ::prost::alloc::vec::Vec<CompilationResult>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations which could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `GetCompilationResult` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCompilationResultRequest {
    /// Required. The compilation result's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// `CreateCompilationResult` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCompilationResultRequest {
    /// Required. The repository in which to create the compilation result. Must be in the
    /// format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The compilation result to create.
    #[prost(message, optional, tag="2")]
    pub compilation_result: ::core::option::Option<CompilationResult>,
}
/// Represents an action identifier. If the action writes output, the output
/// will be written to the referenced database object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Target {
    /// The action's database (Google Cloud project ID) .
    #[prost(string, tag="1")]
    pub database: ::prost::alloc::string::String,
    /// The action's schema (BigQuery dataset ID), within `database`.
    #[prost(string, tag="2")]
    pub schema: ::prost::alloc::string::String,
    /// The action's name, within `database` and `schema`.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
}
/// Describes a relation and its columns.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationDescriptor {
    /// A text description of the relation.
    #[prost(string, tag="1")]
    pub description: ::prost::alloc::string::String,
    /// A list of descriptions of columns within the relation.
    #[prost(message, repeated, tag="2")]
    pub columns: ::prost::alloc::vec::Vec<relation_descriptor::ColumnDescriptor>,
    /// A set of BigQuery labels that should be applied to the relation.
    #[prost(btree_map="string, string", tag="3")]
    pub bigquery_labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Nested message and enum types in `RelationDescriptor`.
pub mod relation_descriptor {
    /// Describes a column.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ColumnDescriptor {
        /// The identifier for the column. Each entry in `path` represents one level
        /// of nesting.
        #[prost(string, repeated, tag="1")]
        pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// A textual description of the column.
        #[prost(string, tag="2")]
        pub description: ::prost::alloc::string::String,
        /// A list of BigQuery policy tags that will be applied to the column.
        #[prost(string, repeated, tag="3")]
        pub bigquery_policy_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Represents a single Dataform action in a compilation result.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompilationResultAction {
    /// This action's identifier. Unique within the compilation result.
    #[prost(message, optional, tag="1")]
    pub target: ::core::option::Option<Target>,
    /// The action's identifier if the project had been compiled without any
    /// overrides configured. Unique within the compilation result.
    #[prost(message, optional, tag="2")]
    pub canonical_target: ::core::option::Option<Target>,
    /// The full path including filename in which this action is located, relative
    /// to the workspace root.
    #[prost(string, tag="3")]
    pub file_path: ::prost::alloc::string::String,
    #[prost(oneof="compilation_result_action::CompiledObject", tags="4, 5, 6, 7")]
    pub compiled_object: ::core::option::Option<compilation_result_action::CompiledObject>,
}
/// Nested message and enum types in `CompilationResultAction`.
pub mod compilation_result_action {
    /// Represents a database relation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Relation {
        /// A list of actions that this action depends on.
        #[prost(message, repeated, tag="1")]
        pub dependency_targets: ::prost::alloc::vec::Vec<super::Target>,
        /// Whether this action is disabled (i.e. should not be run).
        #[prost(bool, tag="2")]
        pub disabled: bool,
        /// Arbitrary, user-defined tags on this action.
        #[prost(string, repeated, tag="3")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Descriptor for the relation and its columns.
        #[prost(message, optional, tag="4")]
        pub relation_descriptor: ::core::option::Option<super::RelationDescriptor>,
        /// The type of this relation.
        #[prost(enumeration="relation::RelationType", tag="5")]
        pub relation_type: i32,
        /// The SELECT query which returns rows which this relation should contain.
        #[prost(string, tag="6")]
        pub select_query: ::prost::alloc::string::String,
        /// SQL statements to be executed before creating the relation.
        #[prost(string, repeated, tag="7")]
        pub pre_operations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// SQL statements to be executed after creating the relation.
        #[prost(string, repeated, tag="8")]
        pub post_operations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Configures `INCREMENTAL_TABLE` settings for this relation. Only set if
        /// `relation_type` is `INCREMENTAL_TABLE`.
        #[prost(message, optional, tag="9")]
        pub incremental_table_config: ::core::option::Option<relation::IncrementalTableConfig>,
        /// The SQL expression used to partition the relation.
        #[prost(string, tag="10")]
        pub partition_expression: ::prost::alloc::string::String,
        /// A list of columns or SQL expressions used to cluster the table.
        #[prost(string, repeated, tag="11")]
        pub cluster_expressions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Sets the partition expiration in days.
        #[prost(int32, tag="12")]
        pub partition_expiration_days: i32,
        /// Specifies whether queries on this table must include a predicate filter
        /// that filters on the partitioning column.
        #[prost(bool, tag="13")]
        pub require_partition_filter: bool,
        /// Additional options that will be provided as key/value pairs into the
        /// options clause of a create table/view statement. See
        /// <https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language>
        /// for more information on which options are supported.
        #[prost(btree_map="string, string", tag="14")]
        pub additional_options: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    }
    /// Nested message and enum types in `Relation`.
    pub mod relation {
        /// Contains settings for relations of type `INCREMENTAL_TABLE`.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IncrementalTableConfig {
            /// The SELECT query which returns rows which should be inserted into the
            /// relation if it already exists and is not being refreshed.
            #[prost(string, tag="1")]
            pub incremental_select_query: ::prost::alloc::string::String,
            /// Whether this table should be protected from being refreshed.
            #[prost(bool, tag="2")]
            pub refresh_disabled: bool,
            /// A set of columns or SQL expressions used to define row uniqueness.
            /// If any duplicates are discovered (as defined by `unique_key_parts`),
            /// only the newly selected rows (as defined by `incremental_select_query`)
            /// will be included in the relation.
            #[prost(string, repeated, tag="3")]
            pub unique_key_parts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// A SQL expression conditional used to limit the set of existing rows
            /// considered for a merge operation (see `unique_key_parts` for more
            /// information).
            #[prost(string, tag="4")]
            pub update_partition_filter: ::prost::alloc::string::String,
            /// SQL statements to be executed before inserting new rows into the
            /// relation.
            #[prost(string, repeated, tag="5")]
            pub incremental_pre_operations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// SQL statements to be executed after inserting new rows into the
            /// relation.
            #[prost(string, repeated, tag="6")]
            pub incremental_post_operations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        /// Indicates the type of this relation.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum RelationType {
            /// Default value. This value is unused.
            Unspecified = 0,
            /// The relation is a table.
            Table = 1,
            /// The relation is a view.
            View = 2,
            /// The relation is an incrementalized table.
            IncrementalTable = 3,
            /// The relation is a materialized view.
            MaterializedView = 4,
        }
        impl RelationType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    RelationType::Unspecified => "RELATION_TYPE_UNSPECIFIED",
                    RelationType::Table => "TABLE",
                    RelationType::View => "VIEW",
                    RelationType::IncrementalTable => "INCREMENTAL_TABLE",
                    RelationType::MaterializedView => "MATERIALIZED_VIEW",
                }
            }
        }
    }
    /// Represents a list of arbitrary database operations.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Operations {
        /// A list of actions that this action depends on.
        #[prost(message, repeated, tag="1")]
        pub dependency_targets: ::prost::alloc::vec::Vec<super::Target>,
        /// Whether this action is disabled (i.e. should not be run).
        #[prost(bool, tag="2")]
        pub disabled: bool,
        /// Arbitrary, user-defined tags on this action.
        #[prost(string, repeated, tag="3")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Descriptor for any output relation and its columns. Only set if
        /// `has_output` is true.
        #[prost(message, optional, tag="6")]
        pub relation_descriptor: ::core::option::Option<super::RelationDescriptor>,
        /// A list of arbitrary SQL statements that will be executed without
        /// alteration.
        #[prost(string, repeated, tag="4")]
        pub queries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Whether these operations produce an output relation.
        #[prost(bool, tag="5")]
        pub has_output: bool,
    }
    /// Represents an assertion upon a SQL query which is required return zero
    /// rows.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Assertion {
        /// A list of actions that this action depends on.
        #[prost(message, repeated, tag="1")]
        pub dependency_targets: ::prost::alloc::vec::Vec<super::Target>,
        /// The parent action of this assertion. Only set if this assertion was
        /// automatically generated.
        #[prost(message, optional, tag="5")]
        pub parent_action: ::core::option::Option<super::Target>,
        /// Whether this action is disabled (i.e. should not be run).
        #[prost(bool, tag="2")]
        pub disabled: bool,
        /// Arbitrary, user-defined tags on this action.
        #[prost(string, repeated, tag="3")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The SELECT query which must return zero rows in order for this assertion
        /// to succeed.
        #[prost(string, tag="4")]
        pub select_query: ::prost::alloc::string::String,
        /// Descriptor for the assertion's automatically-generated view and its
        /// columns.
        #[prost(message, optional, tag="6")]
        pub relation_descriptor: ::core::option::Option<super::RelationDescriptor>,
    }
    /// Represents a relation which is not managed by Dataform but which may be
    /// referenced by Dataform actions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Declaration {
        /// Descriptor for the relation and its columns. Used as documentation only,
        /// i.e. values here will result in no changes to the relation's metadata.
        #[prost(message, optional, tag="1")]
        pub relation_descriptor: ::core::option::Option<super::RelationDescriptor>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CompiledObject {
        /// The database relation created/updated by this action.
        #[prost(message, tag="4")]
        Relation(Relation),
        /// The database operations executed by this action.
        #[prost(message, tag="5")]
        Operations(Operations),
        /// The assertion executed by this action.
        #[prost(message, tag="6")]
        Assertion(Assertion),
        /// The declaration declared by this action.
        #[prost(message, tag="7")]
        Declaration(Declaration),
    }
}
/// `QueryCompilationResultActions` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCompilationResultActionsRequest {
    /// Required. The compilation result's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Maximum number of compilation results to return. The server may return
    /// fewer items than requested. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `QueryCompilationResultActions` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `QueryCompilationResultActions` must match the call that provided the page
    /// token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Optional filter for the returned list. Filtering is only currently
    /// supported on the `file_path` field.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// `QueryCompilationResultActions` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCompilationResultActionsResponse {
    /// List of compilation result actions.
    #[prost(message, repeated, tag="1")]
    pub compilation_result_actions: ::prost::alloc::vec::Vec<CompilationResultAction>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Represents a single invocation of a compilation result.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowInvocation {
    /// Output only. The workflow invocation's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The name of the compilation result to compile. Must be in the format
    /// `projects/*/locations/*/repositories/*/compilationResults/*`.
    #[prost(string, tag="2")]
    pub compilation_result: ::prost::alloc::string::String,
    /// Immutable. If left unset, a default InvocationConfig will be used.
    #[prost(message, optional, tag="3")]
    pub invocation_config: ::core::option::Option<workflow_invocation::InvocationConfig>,
    /// Output only. This workflow invocation's current state.
    #[prost(enumeration="workflow_invocation::State", tag="4")]
    pub state: i32,
    /// Output only. This workflow invocation's timing details.
    #[prost(message, optional, tag="5")]
    pub invocation_timing: ::core::option::Option<super::super::super::r#type::Interval>,
}
/// Nested message and enum types in `WorkflowInvocation`.
pub mod workflow_invocation {
    /// Includes various configuration options for this workflow invocation.
    /// If both `included_targets` and `included_tags` are unset, all actions
    /// will be included.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InvocationConfig {
        /// Immutable. The set of action identifiers to include.
        #[prost(message, repeated, tag="1")]
        pub included_targets: ::prost::alloc::vec::Vec<super::Target>,
        /// Immutable. The set of tags to include.
        #[prost(string, repeated, tag="2")]
        pub included_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Immutable. When set to true, transitive dependencies of included actions will be
        /// executed.
        #[prost(bool, tag="3")]
        pub transitive_dependencies_included: bool,
        /// Immutable. When set to true, transitive dependents of included actions will be
        /// executed.
        #[prost(bool, tag="4")]
        pub transitive_dependents_included: bool,
        /// Immutable. When set to true, any incremental tables will be fully refreshed.
        #[prost(bool, tag="5")]
        pub fully_refresh_incremental_tables_enabled: bool,
    }
    /// Represents the current state of a workflow invocation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The workflow invocation is currently running.
        Running = 1,
        /// The workflow invocation succeeded. A terminal state.
        Succeeded = 2,
        /// The workflow invocation was cancelled. A terminal state.
        Cancelled = 3,
        /// The workflow invocation failed. A terminal state.
        Failed = 4,
        /// The workflow invocation is being cancelled, but some actions are still
        /// running.
        Canceling = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Running => "RUNNING",
                State::Succeeded => "SUCCEEDED",
                State::Cancelled => "CANCELLED",
                State::Failed => "FAILED",
                State::Canceling => "CANCELING",
            }
        }
    }
}
/// `ListWorkflowInvocations` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkflowInvocationsRequest {
    /// Required. The parent resource of the WorkflowInvocation type. Must be in the
    /// format `projects/*/locations/*/repositories/*`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of workflow invocations to return. The server may return
    /// fewer items than requested. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `ListWorkflowInvocations` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListWorkflowInvocations`
    /// must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// `ListWorkflowInvocations` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkflowInvocationsResponse {
    /// List of workflow invocations.
    #[prost(message, repeated, tag="1")]
    pub workflow_invocations: ::prost::alloc::vec::Vec<WorkflowInvocation>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations which could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `GetWorkflowInvocation` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkflowInvocationRequest {
    /// Required. The workflow invocation resource's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// `CreateWorkflowInvocation` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkflowInvocationRequest {
    /// Required. The parent resource of the WorkflowInvocation type.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The workflow invocation resource to create.
    #[prost(message, optional, tag="2")]
    pub workflow_invocation: ::core::option::Option<WorkflowInvocation>,
}
/// `DeleteWorkflowInvocation` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkflowInvocationRequest {
    /// Required. The workflow invocation resource's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// `CancelWorkflowInvocation` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelWorkflowInvocationRequest {
    /// Required. The workflow invocation resource's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents a single action in a workflow invocation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowInvocationAction {
    /// Output only. This action's identifier. Unique within the workflow invocation.
    #[prost(message, optional, tag="1")]
    pub target: ::core::option::Option<Target>,
    /// Output only. The action's identifier if the project had been compiled without any
    /// overrides configured. Unique within the compilation result.
    #[prost(message, optional, tag="2")]
    pub canonical_target: ::core::option::Option<Target>,
    /// Output only. This action's current state.
    #[prost(enumeration="workflow_invocation_action::State", tag="4")]
    pub state: i32,
    /// Output only. If and only if action's state is FAILED a failure reason is set.
    #[prost(string, tag="7")]
    pub failure_reason: ::prost::alloc::string::String,
    /// Output only. This action's timing details.
    /// `start_time` will be set if the action is in [RUNNING, SUCCEEDED,
    /// CANCELLED, FAILED] state.
    /// `end_time` will be set if the action is in [SUCCEEDED, CANCELLED, FAILED]
    /// state.
    #[prost(message, optional, tag="5")]
    pub invocation_timing: ::core::option::Option<super::super::super::r#type::Interval>,
    /// Output only. The workflow action's bigquery action details.
    #[prost(message, optional, tag="6")]
    pub bigquery_action: ::core::option::Option<workflow_invocation_action::BigQueryAction>,
}
/// Nested message and enum types in `WorkflowInvocationAction`.
pub mod workflow_invocation_action {
    /// Represents a workflow action that will run against BigQuery.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQueryAction {
        /// Output only. The generated BigQuery SQL script that will be executed.
        #[prost(string, tag="1")]
        pub sql_script: ::prost::alloc::string::String,
    }
    /// Represents the current state of an workflow invocation action.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The action has not yet been considered for invocation.
        Pending = 0,
        /// The action is currently running.
        Running = 1,
        /// Execution of the action was skipped because upstream dependencies did not
        /// all complete successfully. A terminal state.
        Skipped = 2,
        /// Execution of the action was disabled as per the configuration of the
        /// corresponding compilation result action. A terminal state.
        Disabled = 3,
        /// The action succeeded. A terminal state.
        Succeeded = 4,
        /// The action was cancelled. A terminal state.
        Cancelled = 5,
        /// The action failed. A terminal state.
        Failed = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Pending => "PENDING",
                State::Running => "RUNNING",
                State::Skipped => "SKIPPED",
                State::Disabled => "DISABLED",
                State::Succeeded => "SUCCEEDED",
                State::Cancelled => "CANCELLED",
                State::Failed => "FAILED",
            }
        }
    }
}
/// `QueryWorkflowInvocationActions` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWorkflowInvocationActionsRequest {
    /// Required. The workflow invocation's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Maximum number of workflow invocations to return. The server may return
    /// fewer items than requested. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. Page token received from a previous `QueryWorkflowInvocationActions` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `QueryWorkflowInvocationActions` must match the call that provided the page
    /// token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// `QueryWorkflowInvocationActions` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWorkflowInvocationActionsResponse {
    /// List of workflow invocation actions.
    #[prost(message, repeated, tag="1")]
    pub workflow_invocation_actions: ::prost::alloc::vec::Vec<WorkflowInvocationAction>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod dataform_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Dataform is a service to develop, create, document, test, and update curated
    /// tables in BigQuery.
    #[derive(Debug, Clone)]
    pub struct DataformClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataformClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DataformClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DataformClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Lists Repositories in a given project and location.
        pub async fn list_repositories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRepositoriesRequest>,
        ) -> Result<tonic::Response<super::ListRepositoriesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/ListRepositories",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Fetches a single Repository.
        pub async fn get_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRepositoryRequest>,
        ) -> Result<tonic::Response<super::Repository>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/GetRepository",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Repository in a given project and location.
        pub async fn create_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRepositoryRequest>,
        ) -> Result<tonic::Response<super::Repository>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/CreateRepository",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a single Repository.
        pub async fn update_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRepositoryRequest>,
        ) -> Result<tonic::Response<super::Repository>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/UpdateRepository",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Repository.
        pub async fn delete_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRepositoryRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/DeleteRepository",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Fetches a Repository's remote branches.
        pub async fn fetch_remote_branches(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchRemoteBranchesRequest>,
        ) -> Result<tonic::Response<super::FetchRemoteBranchesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/FetchRemoteBranches",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Workspaces in a given Repository.
        pub async fn list_workspaces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkspacesRequest>,
        ) -> Result<tonic::Response<super::ListWorkspacesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/ListWorkspaces",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Fetches a single Workspace.
        pub async fn get_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkspaceRequest>,
        ) -> Result<tonic::Response<super::Workspace>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/GetWorkspace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Workspace in a given Repository.
        pub async fn create_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkspaceRequest>,
        ) -> Result<tonic::Response<super::Workspace>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/CreateWorkspace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Workspace.
        pub async fn delete_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkspaceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/DeleteWorkspace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Installs dependency NPM packages (inside a Workspace).
        pub async fn install_npm_packages(
            &mut self,
            request: impl tonic::IntoRequest<super::InstallNpmPackagesRequest>,
        ) -> Result<tonic::Response<super::InstallNpmPackagesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/InstallNpmPackages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Pulls Git commits from the Repository's remote into a Workspace.
        pub async fn pull_git_commits(
            &mut self,
            request: impl tonic::IntoRequest<super::PullGitCommitsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/PullGitCommits",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Pushes Git commits from a Workspace to the Repository's remote.
        pub async fn push_git_commits(
            &mut self,
            request: impl tonic::IntoRequest<super::PushGitCommitsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/PushGitCommits",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Fetches Git statuses for the files in a Workspace.
        pub async fn fetch_file_git_statuses(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchFileGitStatusesRequest>,
        ) -> Result<
            tonic::Response<super::FetchFileGitStatusesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/FetchFileGitStatuses",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Fetches Git ahead/behind against a remote branch.
        pub async fn fetch_git_ahead_behind(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchGitAheadBehindRequest>,
        ) -> Result<tonic::Response<super::FetchGitAheadBehindResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/FetchGitAheadBehind",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Applies a Git commit for uncommitted files in a Workspace.
        pub async fn commit_workspace_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::CommitWorkspaceChangesRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/CommitWorkspaceChanges",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Performs a Git reset for uncommitted files in a Workspace.
        pub async fn reset_workspace_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetWorkspaceChangesRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/ResetWorkspaceChanges",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Fetches Git diff for an uncommitted file in a Workspace.
        pub async fn fetch_file_diff(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchFileDiffRequest>,
        ) -> Result<tonic::Response<super::FetchFileDiffResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/FetchFileDiff",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the contents of a given Workspace directory.
        pub async fn query_directory_contents(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDirectoryContentsRequest>,
        ) -> Result<
            tonic::Response<super::QueryDirectoryContentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/QueryDirectoryContents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a directory inside a Workspace.
        pub async fn make_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::MakeDirectoryRequest>,
        ) -> Result<tonic::Response<super::MakeDirectoryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/MakeDirectory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a directory (inside a Workspace) and all of its contents.
        pub async fn remove_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveDirectoryRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/RemoveDirectory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Moves a directory (inside a Workspace), and all of its contents, to a new
        /// location.
        pub async fn move_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveDirectoryRequest>,
        ) -> Result<tonic::Response<super::MoveDirectoryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/MoveDirectory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the contents of a file (inside a Workspace).
        pub async fn read_file(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadFileRequest>,
        ) -> Result<tonic::Response<super::ReadFileResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/ReadFile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a file (inside a Workspace).
        pub async fn remove_file(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFileRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/RemoveFile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Moves a file (inside a Workspace) to a new location.
        pub async fn move_file(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveFileRequest>,
        ) -> Result<tonic::Response<super::MoveFileResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/MoveFile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Writes to a file (inside a Workspace).
        pub async fn write_file(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteFileRequest>,
        ) -> Result<tonic::Response<super::WriteFileResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/WriteFile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists CompilationResults in a given Repository.
        pub async fn list_compilation_results(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCompilationResultsRequest>,
        ) -> Result<
            tonic::Response<super::ListCompilationResultsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/ListCompilationResults",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Fetches a single CompilationResult.
        pub async fn get_compilation_result(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCompilationResultRequest>,
        ) -> Result<tonic::Response<super::CompilationResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/GetCompilationResult",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new CompilationResult in a given project and location.
        pub async fn create_compilation_result(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCompilationResultRequest>,
        ) -> Result<tonic::Response<super::CompilationResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/CreateCompilationResult",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns CompilationResultActions in a given CompilationResult.
        pub async fn query_compilation_result_actions(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCompilationResultActionsRequest>,
        ) -> Result<
            tonic::Response<super::QueryCompilationResultActionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/QueryCompilationResultActions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists WorkflowInvocations in a given Repository.
        pub async fn list_workflow_invocations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkflowInvocationsRequest>,
        ) -> Result<
            tonic::Response<super::ListWorkflowInvocationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/ListWorkflowInvocations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Fetches a single WorkflowInvocation.
        pub async fn get_workflow_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkflowInvocationRequest>,
        ) -> Result<tonic::Response<super::WorkflowInvocation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/GetWorkflowInvocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new WorkflowInvocation in a given Repository.
        pub async fn create_workflow_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkflowInvocationRequest>,
        ) -> Result<tonic::Response<super::WorkflowInvocation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/CreateWorkflowInvocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single WorkflowInvocation.
        pub async fn delete_workflow_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkflowInvocationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/DeleteWorkflowInvocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Requests cancellation of a running WorkflowInvocation.
        pub async fn cancel_workflow_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelWorkflowInvocationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/CancelWorkflowInvocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns WorkflowInvocationActions in a given WorkflowInvocation.
        pub async fn query_workflow_invocation_actions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::QueryWorkflowInvocationActionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::QueryWorkflowInvocationActionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dataform.v1alpha2.Dataform/QueryWorkflowInvocationActions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
