import { OrderService } from "./OrderService";

export class PaymentService {
  constructor(private readonly orders?: OrderService) {}

  capture(): string {
    return this.orders ? "captured-with-order" : "captured";
  }
}

