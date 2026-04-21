/* tslint:disable */
/* eslint-disable */
export type Column = Column;

export type ColumnData = ColumnData;

export type Dataset = Dataset;


export function column_with_label(column: Column, label: string): Column;

export function dataset_with_label(dataset: Dataset, label: string): Dataset;

export function to_xpt(dataset: Dataset): Uint8Array;
