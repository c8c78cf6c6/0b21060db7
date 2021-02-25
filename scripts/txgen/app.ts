import { createObjectCsvWriter } from 'csv-writer';
import { ObjectCsvStringifier } from 'csv-writer/src/lib/csv-stringifiers/object';
import { CsvWriter } from 'csv-writer/src/lib/csv-writer';
import * as random from 'random';

const pareto_shifter =
    (
        alpha,
        range,
    ) => {
        const dist = random.pareto(alpha);

        return () => (
            // normalize pareto into 0 to 1
            // and then project over range
            // and then cast as integer
            (range * (1 - (1 / dist()))) | 0
        );
    };

const client_dist_pareto_alpha = 4; // makes lower num clients have more txs
const tx_dist_pareto_alpha = 2; // makes deposit and withdrawal more likely
const amount_dist_pareto_alpha = 15; // makes smaller amounts more likely

const amount_max = 1_000_000; // seems reasonable for testing
const precision = 10_000; // th of a dollar

enum TxType {
    deposit = 'deposit',
    withdrawal = 'withdrawal',
    dispute = 'dispute',
    resolve = 'resolve',
    chargeback = 'chargeback',
}

const tx_types = [
    TxType.deposit,
    TxType.withdrawal,
    TxType.dispute,
    TxType.resolve,
    TxType.chargeback,
];

type Transaction<NumT> = {
    type: TxType,
    client: NumT,
    tx: NumT,
    amount: NumT | null,
};

const serialize_tx = (tx: Transaction<number>): Transaction<string> => {
    return {
        type: tx.type,
        client: tx.client.toString(),
        tx: tx.tx.toString(),
        amount: tx.amount && (tx.amount / precision).toString() || '',
    };
};

class TxCsvGen {
    private csv_writer: CsvWriter<ObjectCsvStringifier>;

    constructor(
        out_file: string,
    ) {
        this.csv_writer = createObjectCsvWriter({
            header: [
                { id: 'type', title: 'type' },
                { id: 'client', title: 'client' },
                { id: 'tx', title: 'tx' },
                { id: 'amount', title: 'amount' },
            ],
            path: out_file,
        });
    }

    public write_line(tx: Transaction<number>) {
        // the typings are broken
        return this.csv_writer
            .writeRecords([
                serialize_tx(tx) as any,
            ]);
    }

    static async run() {
        if (process.argv.length !== 4) {
            console.log('Usage: <script> num_lines out_file');

            return;
        }

        const [num_lines_raw, out_file] = process.argv.slice(-2);

        const num_lines = parseInt(num_lines_raw, 10) % 0xFFFFFFFF; // u32

        if (isNaN(num_lines)) {
            console.error('Could not parse line amount.');

            return;
        }

        const client_rand =
            pareto_shifter(
                client_dist_pareto_alpha,
                num_lines,
            );

        const tx_rand =
            pareto_shifter(
                tx_dist_pareto_alpha,
                tx_types.length,
            );

        const amount_rand =
            pareto_shifter(
                amount_dist_pareto_alpha,
                amount_max,
            );

        const tx_csv_gen =
            new TxCsvGen(
                out_file,
            );

        const last_deposit_txs: Map<number, [number, number]> = new Map();
        const disputed_txs: Map<number, Set<number>> = new Map();

        for (let i = 0; i < num_lines; ++i) {
            if (i % 1000 === 0) {
                process.stdout.write('\r' + i);
            }

            const client_id = client_rand() % 0xFFFF; // u16
            const type = tx_types[tx_rand() | 0];

            if (type === TxType.deposit) {
                const amount = amount_rand();

                last_deposit_txs.set(
                    client_id,
                    [i, amount],
                );

                await tx_csv_gen.write_line({
                    type,
                    client: client_id,
                    tx: i,
                    amount,
                });

                continue;
            }

            if (type === TxType.withdrawal) {
                let last_deposit = last_deposit_txs.get(client_id);

                // no deposit yet, bail out
                if (!last_deposit) {
                    i -= 1;

                    continue;
                }

                const [_, last_amount] = last_deposit;

                await tx_csv_gen.write_line({
                    type,
                    client: client_id,
                    tx: i,
                    amount: (amount_rand() % last_amount) || last_amount,
                });

                continue;
            }

            if (type == TxType.dispute) {
                let last_deposit = last_deposit_txs.get(client_id);

                // no deposit yet, bail out
                if (!last_deposit) {
                    i -= 1;

                    continue;
                }

                const [tx] = last_deposit;

                // add to disputed txs
                {
                    const disputed = disputed_txs.get(client_id) || new Set();

                    disputed.add(tx);

                    disputed_txs.set(client_id, disputed);
                }

                await tx_csv_gen.write_line({
                    type,
                    client: client_id,
                    tx,
                    amount: null,
                });
            }

            if (type == TxType.chargeback || type == TxType.resolve) {
                const disputed = disputed_txs.get(client_id) || new Set();

                const random_tx = Array.from(disputed)[(Math.random() * disputed.size) | 0];

                if (!random_tx) {
                    i -= 1;

                    continue;
                }

                disputed.delete(random_tx);
                disputed_txs.set(client_id, disputed);

                await tx_csv_gen.write_line({
                    type,
                    client: client_id,
                    tx: random_tx,
                    amount: null,
                });
            }
        }

        console.log('OK');
    }
}

TxCsvGen.run();
