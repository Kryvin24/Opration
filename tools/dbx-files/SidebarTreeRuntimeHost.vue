<script setup lang="ts">
import { computed, nextTick, watch, onBeforeUnmount, onScopeDispose, inject, reactive, ref, shallowRef } from "vue";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog";
import PluginWorkbenchHost from "@/components/plugins/PluginWorkbenchHost.vue";
import type { PluginWorkbenchContext } from "@/lib/plugins/pluginHostBridge";
import { createRoutedSidebarDialogController, routedCanSetCreateDatabaseCharset } from "./sidebarDialogControllerRouting";
import { useSqlHighlighter } from "@/composables/useSqlHighlighter";
import { useSidebarDataOpenRuntime } from "@/composables/useSidebarDataOpenRuntime";
import { useSidebarConnectionMutationRuntime } from "@/composables/useSidebarConnectionMutationRuntime";
import { useSidebarDatabaseSpecificMutationRuntime } from "@/composables/useSidebarDatabaseSpecificMutationRuntime";
import { useSidebarTableMutationRuntime } from "@/composables/useSidebarTableMutationRuntime";
import { useSidebarTreeExportRuntime } from "@/composables/useSidebarTreeExportRuntime";
import { useSidebarTreeToolRuntime } from "@/composables/useSidebarTreeToolRuntime";
import { useI18n } from "vue-i18n";
import { translateBackendError } from "@/i18n/backend-errors";
import {
  BarChart3,
  BookOpen,
  Braces,
  Database,
  ChevronsDown,
  FolderOpen,
  Trash2,
  TerminalSquare,
  RefreshCw,
  Copy,
  TableProperties,
  ListTree,
  Pencil,
  Play,
  Plug,
  Unplug,
  Pin,
  PlugZap,
  ArrowRightLeft,
  Download,
  Eye,
  Upload,
  FileCode,
  FileText,
  Network,
  PencilRuler,
  Search,
  FolderInput,
  FolderPlus,
  Eraser,
  Scissors,
  CopyPlus,
  Plus,
  ScrollText,
  Code2,
  Wrench,
  ListFilter,
  Clipboard,
  UsersRound,
  ShieldCheck,
  Activity,
  Gauge,
  CalendarClock,
  HardDriveDownload,
  FilePlus,
  SquarePen,
  ListX,
  Info,
  X,
  Settings2,
  GitBranch,
  Sparkles,
  Link2,
} from "@lucide/vue";
import type { ContextMenuItem } from "@/components/ui/CustomContextMenu.vue";
import { CONNECTION_ATTEMPT_CANCELLED_MESSAGE, useConnectionStore } from "@/stores/connectionStore";
import { useQueryStore } from "@/stores/queryStore";
import { useSettingsStore } from "@/stores/settingsStore";
import { useSavedSqlStore } from "@/stores/savedSqlStore";
import { savedSqlErrorMessage } from "@/lib/savedSql/savedSqlErrors";
import { useToast } from "@/composables/useToast";
import { createFrontendPluginRegistry } from "@/lib/plugins/frontendPlugin";
import { activatePluginContextMenuItem, buildPluginConnectionContextMenuInvocation, buildPluginTableContextMenuInvocation } from "@/lib/plugins/pluginContext";
import { parseDynamicMenuResponse, renderDynamicMenuEntries, type DynamicMenuAction } from "@/lib/plugins/dynamicContextMenu";
import type { PluginContextMenuInvocation } from "@/lib/plugins/pluginContext";
import type { InstalledPlugin, PluginContextMenuContribution, PluginWorkbenchContribution } from "@/types/database";
import { useDatabaseOptions } from "@/composables/useDatabaseOptions";
import type { ColumnInfo, ConnectionConfig, DatabaseType, TreeNode, TreeNodeType } from "@/types/database";
import * as api from "@/lib/backend/api";
import type { ElasticsearchIndexMetadataKind } from "@/lib/backend/tauri";
import { queryTimeoutSecsForConnection } from "@/lib/sql/queryTimeout";
import { resolveDefaultDatabase } from "@/lib/database/defaultDatabase";
import { connectionUsesVisibleSchemaFilter } from "@/lib/database/visibleDatabases";
import { canTreeNodePin, canTreeNodeShowExpander } from "@/lib/sidebar/sidebarTreeItemLayout";
import { sidebarConnectionVisibleFilterMenu } from "@/lib/sidebar/sidebarVisibleFilterMenu";
import { supportsSidebarObjectNameFilter } from "@/lib/sidebar/sidebarObjectNameFilter";
import { connectionGroupDestinationRows } from "@/lib/sidebar/sidebarLayout";
import {
  hasTableTreeLoadMore,
  hasTableVGroupEntries,
  isTableVGroupContainerNode,
  isTableVGroupGroupableRowType,
  resolveTableVGroupScopeFromNode,
  selectedTableVGroupMoveTargets,
  tableVGroupDestinationRows,
  tableVGroupPathForTable,
  tableVGroupScopeKey,
  tableVGroupsEnabled,
} from "@/lib/table/tableVGroup";
import { objectTypesForGroupNode } from "@/lib/table/tableTree";
import { loadSidebarObjectGroup } from "@/lib/sidebar/sidebarObjectGroupRouting";
import { requestObjectBrowserSearchFocus } from "@/lib/tabs/objectBrowserSearchFocus";
import { isXuguTypeMemberContainer } from "@/lib/sidebar/xuguTypeMembers";
import { isXuguSyntheticTreeNode } from "@/lib/sidebar/xuguPublicSynonyms";
import { buildXuguSchedulerJobSql, type XuguSchedulerJobAction } from "@/lib/database/xuguSchedulerJobSql";
import { canViewDatabaseObjectDependencies, databaseDependencyProviderFor } from "@/lib/database/databaseObjectDependencies";
import { elasticsearchClearIndexPreview, isElasticsearchClearConfirmed, isElasticsearchIndexPattern } from "@/lib/sidebar/elasticsearchIndexActions";
import { mysqlObjectTemplateForGroup } from "@/lib/sidebar/mysqlObjectTemplates";
import { buildTableDeleteTemplate, buildTableInsertTemplate, buildTableSelectTemplate, buildTableUpdateTemplate } from "@/lib/table/tableSqlTemplates";
import { joinExportedDdls } from "@/lib/export/ddlExport";
import { qualifiedTableName } from "@/lib/table/tableSelectSql";
import { driverStoreFocusForInstallError } from "@/lib/connection/agentDriverInstallHint";
import {
  canCreateConnectionNamespace,
  canCreateDatabaseNodeNamespace,
  canEditDatabaseProperties as canEditDatabasePropertiesForNode,
  connectionNamespaceCreationTarget,
  editableDatabasePropertyGroups,
  supportsDatabaseCreation,
  supportsDatabaseSearch,
  supportsConnectionDatabaseBrowser,
  supportsConnectionQueryActions,
  supportsAiAssistantContext,
  supportsFieldLineage,
  supportsObjectBrowserTreeNode,
  supportsDataDictionary,
  supportsSchemaDiagram,
  supportsSqlFileExecution,
  supportsTableImport,
  supportsTableTruncate,
  supportsTableStructureEditing,
  supportsTransfer,
  supportsPackageMemberExpansion,
  usesTreeSchemaMode,
  isSingleDatabase,
  schemaNodeHasLoadableName,
} from "@/lib/database/databaseCapabilities";
import { copyDisplayPathForTreeNode, copyNameForTreeNode, isDirectNavigationTreeNode, isDocumentBrowserTreeNode, isRepeatableNavigationTreeNode, objectSourceTargetForTreeNode, shouldRunTreeNodeRowAction, treeNodeRowAction, treeNodeRowDoubleClickAction } from "@/lib/sidebar/treeNodeClick";
import { customTypeCapabilities, supportsTypeObjectSource } from "@/lib/database/databaseObjectCapabilities";
import { mongoCollectionTableTypeFromNode, mongoCreateDatabasePreview, mongoDropIndexFailureCount } from "@/lib/sidebar/mongoCollectionMutation";
import { dataTabOpenModeFromTreeClick, type DataTabOpenMode } from "@/lib/sidebar/dataTabOpenPolicy";
import { isCopySidebarSelectionShortcut, isEditSidebarConnectionShortcut, isModRShortcut, isPasteSidebarSelectionShortcut } from "@/lib/editor/keyboardShortcuts";
import { handleSidebarTreeDeleteShortcut } from "@/lib/sidebar/sidebarTreeDeleteShortcut";
import { dataTableDoubleClickAction } from "@/lib/tabs/dataTabActivation";
import { attachedDatabaseNameFromPath, buildCreateDatabaseSql, buildDuckDbAttachDatabaseSql, buildSqliteAttachDatabaseSql, supportsCreateDatabaseCharset, supportsCreateDatabaseLocale, uniqueAttachedDatabaseName } from "@/lib/database/createDatabaseSql";
import { appendCreateDatabaseErrorHint } from "@/lib/database/createDatabaseErrorHints";
import { SQLITE_DATABASE_FILE_EXTENSIONS } from "@/lib/database/databaseFileDetection";
import {
  buildCreateSchemaSql,
  buildDropDatabaseSql,
  buildDropObjectSql,
  buildDropSchemaSql,
  damengDropSchemaExecutionSchema,
  buildGetDatabaseCommentSql,
  buildGetSchemaCommentSql,
  buildUpdateDatabasePropertiesSql,
  buildDropTableSql,
  buildDropTableChildObjectSql,
  buildDuplicateTableStructurePlan,
  buildCopyTableDataSql,
  buildEmptyTableSql,
  buildTruncateTableSql,
  buildMysqlAutoIncrementSql,
  supportsDropTableCascade,
  supportsTruncateTableCascade,
  supportsNativeMysqlAutoIncrement,
  supportsSchemaComment,
  type DropTableChildObjectSqlOptions,
  type DropObjectSqlOptions,
  type MysqlAutoIncrementSqlOptions,
  type TableChildObjectType,
} from "@/lib/database/dbAdminSql";
import { buildRenameObjectSql, buildRenameDatabaseSql, buildRenameDatabasePreflightSql, databaseRenameMaintenanceDatabase, supportsDatabaseRename, supportsObjectRename, type RenameableObjectType } from "@/lib/table/objectRenameSql";
import { buildRoutineRenameObjectSourceStatements, supportsSourceBackedRoutineRename } from "@/lib/table/objectSourceEditor";
import { buildViewDdl } from "@/lib/table/viewDdl";
import { formatSqlForDisplay, sqlFormatDialectForDbType } from "@/lib/sql/sqlFormatter";
import { getTableStructureCapabilities } from "@/lib/table/tableStructureCapabilities";
import { connectionObjectTreeNodeSchema, connectionObjectTreeQuerySchema, connectionTableSqlSchema, connectionUsesDatabaseObjectTreeMode, effectiveDatabaseTypeForConnection, tableStructureDatabaseTypeForConnection } from "@/lib/database/jdbcDialect";
import { isObjectCacheInvalidationError } from "@/lib/metadata/objectCacheInvalidationError";
import { hasTreeNodeDatabaseContext } from "@/lib/sidebar/treeNodeContext";
import {
  defaultPasteTableMode,
  pasteTableModeCopiesData,
  supportsWholeRowTableDataCopy,
  tableClipboardMatchesTarget,
  tableClipboardMenuState,
  tableClipboardSourceContext,
  tableDataCopyColumnOptions,
  tablePasteFeedback,
  type TableClipboardContext,
  type TableClipboardTableContext,
} from "@/lib/table/tableClipboard";
import { selectedSidebarBatchTargets, selectedTreeNodesInVisibleOrder as orderSelectedTreeNodes } from "@/lib/sidebar/sidebarTreeSelection";
import { connectionPasteTargetGroupId, selectedConnectionClipboardTargets, selectedConnectionEditTarget, selectedConnectionMoveTargets } from "@/lib/sidebar/sidebarConnectionSelection";
import { connectionSupportsDatabaseUserAdmin, resolveDatabaseUserAdminProviderForConnection, type DatabaseUserIdentity } from "@/lib/database/databaseUserAdmin";
import { authorizationPlanSql, authorizationPlanStatus, buildCreateDatabaseAuthorizationPlan, executeAuthorizationPlan, type AuthorizationPlan, type AuthorizationStepResult } from "@/lib/database/databaseAuthorizationPlan";
import { connectionSupportsProcessList } from "@/lib/database/processListDrivers";
import { connectionSupportsServerDashboard } from "@/lib/database/mysqlServerStatus";
import { connectionSupportsServerDashboard as connectionSupportsPgServerDashboard } from "@/lib/database/postgresServerStatus";
import { connectionSupportsXuguServerDashboard } from "@/lib/database/xuguServerStatus";
import { sidebarTreeContextKey } from "@/lib/sidebar/sidebarTreeContext";
import { sidebarTreeArrowAction } from "@/lib/sidebar/sidebarTreeArrowNavigation";
import { batchTableEmptyFeedback, runBatchTableEmpty } from "@/lib/sidebar/batchTableEmpty";
import { runBatchTableTruncate } from "@/lib/table/batchTableTruncate";
import { runBatchTableDrop } from "@/lib/table/batchTableDrop";
import { buildSidebarDdlTemplateSql, formatSidebarDdlTemplateForDisplay } from "@/lib/sidebar/sidebarDdlTemplate";
import { resolveSidebarDdlTargets } from "@/lib/sidebar/sidebarDdlTargets";
import { sidebarTableDataExportTargets } from "@/lib/sidebar/sidebarExportRuntime";
import { formatSidebarTableCopyText, type FormatSidebarTableNamesOptions } from "@/lib/sidebar/sidebarTableNameCopy";
import { supportsScheduledDatabaseBackup } from "@/lib/backup/scheduledDatabaseBackup";
import { isTauriRuntime } from "@/lib/backend/tauriRuntime";
import { copyToClipboard } from "@/lib/common/clipboard";
import { buildConnectionUrlCopy, CONNECTION_URL_COPY_WITH_PASSWORD_FORMATS, connectionUrlCopyFormats, type ConnectionUrlCopyFormat } from "@/lib/connection/connectionUrlBuilder";
import { rankSavedSqlHistory, type SavedSqlHistoryScope } from "@/lib/savedSql/savedSqlHistory";
import { savedSqlClipboardFileIds, savedSqlPasteTargetForNode } from "@/lib/savedSql/savedSqlClipboard";
import { exportSavedSqlFileContent } from "@/lib/savedSql/savedSqlExport";
import { isSqlServerLinkedNode } from "@/lib/database/sqlServerLinkedServers";
import { flattenTree } from "@/composables/useFlatTree";
import { createDatabaseCollationOptionsForCharset, DEFAULT_GBASE8S_DATABASE_LOCALE, defaultGbase8sDatabaseLocale, GBASE8S_DATABASE_LOCALES, nextCreateDatabaseCollation, normalizeCreateDatabaseCharset, parseCreateDatabaseCharsetMetadata } from "@/lib/database/createDatabaseCharsetOptions";
import { executeWithProductionContextGuard, executeWithProductionSqlGuard } from "@/lib/database/productionExecutionGuard";
import { connectionIsEffectivelyReadOnly } from "@/lib/database/readOnlyWriteAccess";
import { buildXuguCompileSql } from "@/lib/database/xuguCompileSql";
import { buildDamengCompileViewSql } from "@/lib/database/damengCompileSql";
import type { SidebarDataOpenRequest } from "@/lib/sidebar/sidebarDataOpenCoordinator";
import { createSidebarActionTarget, findSidebarActionTarget, releaseRemovedSidebarActionTarget, type SidebarActionTarget } from "@/lib/sidebar/sidebarActionTarget";
import { createSidebarMenuContext, normalizeSidebarMenuDescriptors } from "@/lib/sidebar/sidebarTreeMenuDescriptors";
import { driverProfileDatabaseWorkspace } from "@/lib/database/driverProfileExtensions";
import type { SidebarDangerDialogRequest } from "@/lib/sidebar/sidebarDangerDialog";
import {
  fallbackCreateDatabaseCharset,
  sidebarTreeDialogOwner,
  sidebarDangerTarget,
  sidebarDangerRunningCancel,
  sidebarFormTarget,
  showDeleteConfirm,
  showTableVGroupDialog,
  showTableVGroupDeleteConfirm,
  tableVGroupDeleteTarget,
  tableVGroupName,
  tableVGroupDialogScope,
  tableVGroupDialogParentGroupId,
  tableVGroupDialogTableNames,
  showDropTableConfirm,
  showDropTableChildObjectConfirm,
  showBatchDropConfirm,
  showBatchEmptyConfirm,
  showBatchTruncateConfirm,
  showStructurePreviewDialog,
  showStructureDocCopyDialog,
  structurePreviewSql,
  structurePreviewTitle,
  structurePreviewError,
  structureDocCopyText,
  structureDocCopyTitle,
  isLoadingStructurePreview,
  showEmptyTableConfirm,
  showTruncateTableConfirm,
  showVacuumTableConfirm,
  showMysqlAutoIncrementConfirm,
  showBatchMysqlAutoIncrementConfirm,
  batchMysqlAutoIncrementTargets,
  batchMysqlAutoIncrementPreviewSql,
  showRenameObjectDialog,
  renameObjectName,
  renameObjectError,
  renameObjectPreviewSql,
  dropTablePreviewSql,
  dropTableCascade,
  batchDropCascade,
  emptyTablePreviewSql,
  truncateTablePreviewSql,
  truncateTableCascade,
  vacuumTableFull,
  vacuumTableAnalyze,
  vacuumTablePreviewSql,
  vacuumTableExecuting,
  mysqlAutoIncrementValue,
  mysqlAutoIncrementPreviewSql,
  dropObjectPreviewSql,
  showDropObjectConfirm,
  dropTableChildObjectPreviewSql,
  batchDropPreviewSql,
  batchEmptyPreviewSql,
  batchEmptyTargets,
  batchDropTargets,
  batchTruncateTargets,
  batchTruncatePreviewSql,
  batchTruncateCascade,
  dropDatabasePreviewSql,
  dropSchemaPreviewSql,
  showDuplicateDialog,
  duplicateTableName,
  duplicateStructureSource,
  showPasteDialog,
  pasteTableMode,
  pasteTableEntries,
  showCreateDatabaseDialog,
  createDatabaseName,
  createDatabaseCharset,
  createDatabaseCollation,
  createDatabaseUsers,
  createDatabaseSelectedUsers,
  createDatabaseUsersLoading,
  showCreateDatabasePreviewDialog,
  createDatabaseAuthorizationPlan,
  createDatabasePreviewSql,
  createDatabaseAuthorizationResults,
  createDatabaseAuthorizationApplying,
  showCreateNacosNamespaceDialog,
  createNacosNamespaceId,
  createNacosNamespaceName,
  createNacosNamespaceDesc,
  createNacosNamespaceLoading,
  showEditNacosNamespaceDialog,
  editNacosNamespaceName,
  editNacosNamespaceDesc,
  editNacosNamespaceLoading,
  showDeleteNacosNamespaceConfirm,
  deleteNacosNamespaceLoading,
  createDatabaseCharsetOptions,
  createDatabaseCollationsByCharset,
  createDatabaseCharsetLoading,
  showDropDatabaseConfirm,
  dropDatabaseLoading,
  showDropMongoCollectionConfirm,
  dropMongoCollectionLoading,
  showDropMongoIndexConfirm,
  dropMongoIndexLoading,
  showDropAllMongoIndexesConfirm,
  dropAllMongoIndexesLoading,
  showCreateMongoIndexDialog,
  showCreateMeilisearchIndexDialog,
  meilisearchCreateIndexUid,
  meilisearchCreateIndexPrimaryKey,
  meilisearchCreateIndexError,
  meilisearchCreateIndexLoading,
  mongoCreateIndexForm,
  mongoCreateIndexFieldOptions,
  mongoCreateIndexError,
  mongoCreateIndexLoading,
  showMongoIndexManagerDialog,
  mongoIndexManagerRows,
  mongoIndexManagerLoading,
  mongoIndexManagerError,
  mongoIndexManagerSelectedName,
  mongoIndexManagerMode,
  mongoEditIndexOriginalName,
  showClearElasticsearchIndexConfirm,
  clearElasticsearchIndexLoading,
  clearElasticsearchIndexTypedName,
  showFlushRedisDbConfirm,
  showCreateSchemaDialog,
  createSchemaName,
  showDropSchemaConfirm,
  showEditDatabasePropertiesDialog,
  editDatabasePropertiesLoading,
  editDatabasePropertiesPreviewSql,
  editDatabaseCharset,
  editDatabaseCollation,
  editDatabaseCommentText,
  showEditSchemaCommentDialog,
  showCompileErrorDialog,
  compileErrorTitle,
  compileErrorMessage,
  schemaCommentText,
  schemaCommentLoading,
  schemaCommentPreviewSql,
  showDeleteGroupConfirm,
  deleteConnectionsWithGroup,
  showMoveToNewGroupDialog,
  moveToNewGroupName,
  type DuplicateStructureSource,
} from "./sidebarTreeDialogState";

const { t, locale: appLocale } = useI18n();

const connectionStore = useConnectionStore();

const queryStore = useQueryStore();

const settingsStore = useSettingsStore();

const savedSqlStore = useSavedSqlStore();

const { toast } = useToast();
const installedPlugins = ref<InstalledPlugin[]>([]);
const sidebarPluginRegistry = computed(() => createFrontendPluginRegistry(installedPlugins.value, appLocale.value));
const pluginDialog = shallowRef<{ plugin: InstalledPlugin; contribution: PluginWorkbenchContribution; context: PluginWorkbenchContext; title: string } | null>(null);

async function refreshInstalledPlugins() {
  try {
    const plugins = await api.listPlugins();
    installedPlugins.value = plugins.filter((plugin) => plugin.compatibility.compatible);
  } catch {
    // Keep the previous list: an empty registry would drop live plugin context-menu entries.
  }
}

// Plugin install/update/uninstall from the Plugin Center broadcasts this event;
// without it the sidebar registry keeps the mount-time snapshot and the Table /
// Connection context menus stay empty until DBX restarts.
const onPluginsChanged = () => void refreshInstalledPlugins();
window.addEventListener("dbx:plugins-changed", onPluginsChanged);
onScopeDispose(() => window.removeEventListener("dbx:plugins-changed", onPluginsChanged));
void refreshInstalledPlugins();

const { highlight } = useSqlHighlighter();

const { openData } = useSidebarDataOpenRuntime();

const { getDatabaseOptions } = useDatabaseOptions();

const props = defineProps<{
  node: TreeNode;
  depth: number;
  dragDisabled?: boolean;
  pendingRename?: boolean;
  highlighted?: boolean;
}>();

const activeNode = shallowRef<TreeNode>(props.node);
let acceptedSelectionIds: readonly string[] | null = null;
let latestNavigationRequestId = 0;

function beginNavigationRequest(): number {
  latestNavigationRequestId += 1;
  return latestNavigationRequestId;
}

function isCurrentNavigationRequest(requestId: number): boolean {
  return requestId === latestNavigationRequestId;
}

function releaseActiveNodeReference(nodeIds: readonly string[]) {
  activeNode.value = releaseRemovedSidebarActionTarget(activeNode.value, nodeIds);
}

watch(
  () => connectionStore.treeNodes,
  (nodes) => {
    const liveNode = findSidebarActionTarget(nodes, createSidebarActionTarget(activeNode.value));
    activeNode.value = liveNode ?? releaseRemovedSidebarActionTarget(activeNode.value, [activeNode.value.id]);
  },
  { flush: "post" },
);

const { copyStructureAs, copyStructureDocText, copyStructurePreview, exportData, exportDataXlsx, exportMongoCollection, exportStructure, saveStructurePreview, selectTextareaContent } = useSidebarTreeExportRuntime({
  activeNode,
  connectionStore,
  settingsStore,
  acceptedSelectionIds: () => acceptedSelectionIds,
});

const {
  openAllDatabasesExport,
  openDataCompare,
  openDatabaseExport,
  openDatabaseSearch,
  openDataDictionary,
  openDiagram,
  openDocs,
  openFieldLineage,
  openMongoImport,
  openMongoDatabaseDump,
  openScheduledBackups,
  openSchemaDiff,
  openSchemaDiffForRoutine,
  openSqlFileExecution,
  openStructureEditor,
  openTableImport,
  openTransfer,
} = useSidebarTreeToolRuntime({
  activeNode,
  connectionStore,
  queryStore,
  settingsStore,
  tableChildObjectName: tableChildDropObjectName,
  acceptedSelectionIds: () => acceptedSelectionIds,
});

const emit = defineEmits<{
  "rename-started": [];
  "request-connection-rename": [connectionId: string];
  "request-group-rename": [groupId: string];
  "request-saved-sql-rename": [nodeId: string];
  "node-toggled": [node: TreeNode, expanded: boolean];
  "search-toggle": [node: TreeNode];
  "context-menu": [event: MouseEvent, node: TreeNode, items: ContextMenuItem[]];
  "open-ddl": [node: TreeNode];
  "open-elasticsearch-index-metadata": [node: TreeNode, kind: ElasticsearchIndexMetadataKind];
  "open-object-source": [node: TreeNode, initialEditing: boolean];
  "open-procedure": [node: TreeNode];
  "open-settings": [initialTab: string];
  "open-data": [node: TreeNode, requireSelection: boolean, openMode: DataTabOpenMode, runner: (node: TreeNode, request: SidebarDataOpenRequest) => Promise<void>];
  "open-visible-databases": [node: TreeNode];
  "open-visible-schemas": [node: TreeNode];
  "open-visible-nacos-namespaces": [node: TreeNode];
  "open-table-name-filters": [node: TreeNode];
  "add-to-ai": [nodes: TreeNode | TreeNode[]];
  "open-danger-dialog": [request: SidebarDangerDialogRequest];
  "open-dialog-controller": [controller: Record<string, any> | null];
  "open-install-extension": [node: TreeNode];
  "open-extension-details": [node: TreeNode];
  "open-event-trigger-details": [node: TreeNode];
}>();

const {
  setNodeAsDefaultDatabase,
  clearNodeDefaultDatabase,
  setNodeAsDefaultSchema,
  clearNodeDefaultSchema,
  connectionDeleteMenuLabel,
  connectionDuplicateMenuLabel,
  connectionDeleteConfirmMessage,
  deleteConnection,
  confirmDelete,
  copyFinalProxyPort,
  duplicateConnection,
  editConnection,
  revealConnectionFilePath,
  revealDatabaseFile,
  canBackupSqliteDatabase,
  backupSqliteDatabase,
  restoreSqliteDatabase,
  disconnectConnection,
  connectionDisconnectMenuLabel,
  canDisconnectConnection,
  connectionGroupDisconnectMenuLabel,
  canDisconnectConnectionGroup,
  disconnectConnectionGroup,
  canForgetSessionCredential,
  disconnectAndForgetConnectionPassword,
  cancelConnectionAttempt,
  closeDatabaseConnection,
  isPinned,
  isNodeDefaultDatabase,
  isNodeDefaultSchema,
  isConnected,
  isConnecting,
  canCloseDatabaseConnection,
  canConfigureVisibleDatabases,
  canConfigureVisibleSchemas,
  canCopyFinalProxyPort,
  togglePin,
  openVisibleDatabasesDialog,
  openVisibleSchemasDialog,
  startRenameGroup,
  connectionGroupDeleteMenuLabel,
  connectionGroupDeleteConfirmMessage,
  deleteConnectionGroup,
  newConnectionInGroup,
  newSubgroup,
  confirmDeleteGroup,
  deletingConnectionGroups,
  moveToGroup,
  createGroupAndMoveConnection,
} = useSidebarConnectionMutationRuntime({
  activeNode,
  releaseActiveNodeReference,
  selectedTreeNodesInVisibleOrder,
  connectionStore,
  queryStore,
  requestGroupRename: (groupId) => emit("request-group-rename", groupId),
  openVisibleDatabases: (node) => emit("open-visible-databases", node),
  openVisibleSchemas: (node) => emit("open-visible-schemas", node),
});

const {
  canDropMongoDatabase,
  canDropMilvusDatabase,
  canDropMongoCollection,
  canDropMilvusCollection,
  canRenameMongoCollection,
  canCloneMongoCollection,
  prepareRenameMongoCollectionDialog,
  confirmRenameMongoCollection,
  showRenameMongoCollectionDialog,
  renameMongoCollectionName,
  renameMongoCollectionError,
  renameMongoCollectionPreview,
  renameMongoCollectionLoading,
  prepareCloneMongoCollectionDialog,
  confirmCloneMongoCollection,
  showCloneMongoCollectionDialog,
  cloneMongoCollectionName,
  cloneMongoCollectionError,
  cloneMongoCollectionLoading,
  mongoIndexNameForNode,
  canDropMongoIndexNode,
  canDropMongoIndex,
  canDropAllMongoIndexes,
  mongoIndexDropPreview,
  mongoDropAllIndexesPreview,
  refreshMongoIndexTreeAfterMutation,
  canCreateMongoIndex,
  mongoIndexKeyTypes,
  mongoCreateIndexCanSubmit,
  mongoCreateIndexCanAddField,
  prepareCreateMongoIndexDialog,
  canCreateMeilisearchIndex,
  prepareCreateMeilisearchIndexDialog,
  confirmCreateMeilisearchIndex,
  addMongoCreateIndexField,
  removeMongoCreateIndexField,
  confirmCreateMongoIndex,
  canManageMongoIndexes,
  prepareMongoIndexManagerDialog,
  loadMongoIndexManagerRows,
  mongoIndexManagerSelected,
  mongoIndexManagerCollectionName,
  selectMongoIndexRow,
  startCreateMongoIndexDraft,
  startEditMongoIndexDraft,
  cancelMongoIndexDraft,
  dropSelectedMongoIndexRow,
  canDropSelectedMongoIndexRow,
  canEditSelectedMongoIndexRow,
  confirmEditMongoIndex,
  openCreateNacosNamespaceDialog,
  confirmCreateNacosNamespace,
  openEditNacosNamespaceDialog,
  confirmEditNacosNamespace,
  openDeleteNacosNamespaceConfirm,
  confirmDeleteNacosNamespace,
  dropMongoCollection,
  dropMilvusCollection,
  dropMongoIndex,
  dropAllMongoIndexes,
  canManageElasticsearchIndex,
  clearElasticsearchIndex,
  confirmClearElasticsearchIndex,
  flushRedisDb,
  prepareRedisDatabaseAliasDialog,
  confirmRedisDatabaseAlias,
  clearRedisDatabaseAlias,
  showRedisDatabaseAliasDialog,
  redisDatabaseAliasInput,
  redisDatabaseAliasSaving,
  confirmFlushRedisDb,
  confirmDropMongoDatabase,
  confirmDropMongoCollection,
  confirmDropMilvusDatabase,
  confirmDropMilvusCollection,
  confirmDropMongoIndex,
  confirmDropAllMongoIndexes,
} = useSidebarDatabaseSpecificMutationRuntime({ activeNode, connectionStore });

const {
  isTableNotView,
  supportsTruncate,
  supportsVacuum,
  supportsMysqlAutoIncrement,
  canDropTableCascade,
  canTruncateTableCascade,
  refreshDropTablePreviewSql,
  refreshTruncateTablePreviewSql,
  dropTable,
  refreshTableList,
  confirmDropTable,
  emptyTable,
  confirmEmptyTable,
  truncateTable,
  confirmTruncateTable,
  vacuumTable,
  refreshVacuumPreviewForOptions,
  confirmVacuumTable,
  mysqlAutoIncrement,
  refreshMysqlAutoIncrementPreviewSql,
  confirmMysqlAutoIncrement,
} = useSidebarTableMutationRuntime({
  activeNode,
  releaseActiveNodeReference,
  connectionStore,
  currentDatabaseType,
  databaseTypeForNode,
  executeWithProductionGuard: executeTreeNodeSqlWithProductionGuard,
  closeDroppedTableObjectTabsForNode,
  refreshMutatedTableDataTabsForNode,
});

const batchDropProgress = ref({ completed: 0, total: 0 });

const treeItemDialogOwner = Symbol("sidebar-tree-dialog-owner");

function claimTreeItemDialogOwnership() {
  sidebarTreeDialogOwner.value = treeItemDialogOwner;
}

function routeTreeItemDialogController() {
  const controller = getTreeItemDialogController();
  const target = createSidebarActionTarget(activeNode.value);
  sidebarFormTarget.value = target;
  const routedController = createRoutedSidebarDialogController(controller, {
    node: target,
    wrapAction: (action) => {
      return (...args: unknown[]) => {
        activateActionTarget(target);
        return action(...args);
      };
    },
  });
  routedController.pasteTableDataCopySupported = pasteTableDataCopySupported.value;
  routedController.canSetCreateDatabaseCharset = routedCanSetCreateDatabaseCharset(canSetCreateDatabaseCharset.value, canSetCreateDatabaseLocale.value);
  routedController.canEditDatabaseCharsetCollation = canEditDatabaseCharsetCollation.value;
  routedController.canEditDatabaseComment = canEditDatabaseComment.value;
  emit("open-dialog-controller", routedController);
}

const sidebarTreeContext = inject(sidebarTreeContextKey, null);

function shouldReleaseCollapsedTreeNodeChildren(): boolean {
  return !connectionStore.sidebarSearchQuery && !sidebarTreeContext?.isSearchProjectionActive?.();
}

function currentDatabaseType(): DatabaseType | undefined {
  return activeNode.value.connectionId ? effectiveDatabaseTypeForConnection(connectionStore.getConfig(activeNode.value.connectionId)) : undefined;
}

function currentTableStructureDatabaseType(): DatabaseType | undefined {
  return activeNode.value.connectionId ? tableStructureDatabaseTypeForConnection(connectionStore.getConfig(activeNode.value.connectionId)) : undefined;
}

function rawDatabaseType(): DatabaseType | undefined {
  return activeNode.value.connectionId ? connectionStore.getConfig(activeNode.value.connectionId)?.db_type : undefined;
}

function databaseTypeForNode(node: TreeNode): DatabaseType | undefined {
  return node.connectionId ? effectiveDatabaseTypeForConnection(connectionStore.getConfig(node.connectionId)) : undefined;
}

function sidebarTableCopyFormatOptions(node: TreeNode = activeNode.value): FormatSidebarTableNamesOptions {
  const config = node.connectionId ? connectionStore.getConfig(node.connectionId) : undefined;
  return {
    separator: settingsStore.editorSettings.sidebarCopyTableNameSeparator,
    includeSchema: settingsStore.editorSettings.sidebarCopyTableNameIncludeSchema,
    databaseType: node.connectionId ? effectiveDatabaseTypeForConnection(config) : undefined,
    driverProfile: config?.driver_profile,
    identifierQuote: node.connectionId ? connectionStore.connectionIdentifierQuote?.(node.connectionId) : undefined,
  };
}

function formatSelectedTableNamesForClipboard(selectedNodes: readonly TreeNode[] = selectedTreeNodesInVisibleOrder()): string {
  return formatSidebarTableCopyText(activeNode.value, selectedNodes, sidebarTableCopyFormatOptions());
}

function tableStructureDatabaseTypeForNode(node: TreeNode): DatabaseType | undefined {
  return node.connectionId ? tableStructureDatabaseTypeForConnection(connectionStore.getConfig(node.connectionId)) : undefined;
}

function hasNodeDatabaseContext(node: TreeNode): node is TreeNode & { connectionId: string; database: string } {
  return !!node.connectionId && hasTreeNodeDatabaseContext(node);
}

const groupTypes: Set<TreeNodeType> = new Set([
  "group-columns",
  "group-indexes",
  "group-fkeys",
  "group-triggers",
  "group-events",
  "group-constraints",
  "group-table-partitions",
  "group-table-subpartitions",
  "group-tables",
  "group-views",
  "group-materialized-views",
  "group-procedures",
  "group-functions",
  "group-sequences",
  "group-synonyms",
  "group-jobs",
  "group-packages",
  "group-types",
  "group-partitions",
  "group-extensions",
  "group-event-triggers",
  "group-tablespaces",
  "group-datafiles",
]);

function isGroupLabel(node: TreeNode): boolean {
  return groupTypes.has(node.type);
}

async function openDirectNavigationNode(node: TreeNode, requestId: number) {
  if (!node.connectionId) return;
  const isNacosNavigation = node.type === "nacos-namespace" || node.type === "nacos-access-control";
  const isEtcdNavigation = node.type === "etcd-root" || node.type === "etcd-dashboard" || node.type === "etcd-access-control";
  await connectionStore.ensureConnected(node.connectionId, isNacosNavigation || isEtcdNavigation ? { verifyHealth: false } : undefined);
  if (!isCurrentNavigationRequest(requestId)) return;
  if ((node.type === "etcd-dashboard" || node.type === "etcd-access-control") && !connectionStore.getEtcdAccessCapabilities(node.connectionId).admin) return;
  const connectionName = connectionStore.getConfig(node.connectionId)?.name || (isNacosNavigation ? "Nacos" : isEtcdNavigation ? "etcd" : "Consul");
  if (node.type === "consul-root") {
    queryStore.createTab(node.connectionId, "", `${connectionName}:keys`, "consul");
    refreshActiveKvBrowserAfterOpen("consul", node.connectionId);
  } else if (node.type === "consul-overview") {
    queryStore.createTab(node.connectionId, "", `${connectionName}:${t("consul.ui.overview")}`, "consul-overview");
  } else if (node.type === "etcd-root") {
    queryStore.createTab(node.connectionId, "", `${connectionName}:keys`, "etcd");
    refreshActiveKvBrowserAfterOpen("etcd", node.connectionId);
  } else if (node.type === "etcd-dashboard") {
    queryStore.createTab(node.connectionId, "", `${connectionName}:dashboard`, "etcd-dashboard");
  } else if (node.type === "etcd-access-control") {
    queryStore.createTab(node.connectionId, "", `${connectionName}:access-control`, "etcd-access-control");
  } else if (node.type === "nacos-namespace") {
    queryStore.openNacosAdmin(node.connectionId, { namespace: n